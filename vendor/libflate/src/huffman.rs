/// Length-limited Huffman Codes
///
use std::io;
use std::cmp;

use bit;

const MAX_BITWIDTH: u8 = 15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    pub width: u8,
    pub bits: u16,
}
impl Code {
    pub fn new(width: u8, bits: u16) -> Self {
        debug_assert!(width <= MAX_BITWIDTH);
        Code {
            width: width,
            bits: bits,
        }
    }
    fn inverse_endian(&self) -> Self {
        let mut f = self.bits;
        let mut t = 0;
        for _ in 0..self.width {
            t <<= 1;
            t |= f & 1;
            f >>= 1;
        }
        Code::new(self.width, t)
    }
}

pub trait Builder: Sized {
    type Instance;
    fn set_mapping(&mut self, symbol: u16, code: Code);
    fn finish(self) -> Self::Instance;
    fn restore_canonical_huffman_codes(mut self, bitwidthes: &[u8]) -> Self::Instance {
        debug_assert!(!bitwidthes.is_empty());

        let mut symbols = bitwidthes
            .iter()
            .enumerate()
            .filter(|&(_, &code_bitwidth)| code_bitwidth > 0)
            .map(|(symbol, &code_bitwidth)| (symbol as u16, code_bitwidth))
            .collect::<Vec<_>>();
        symbols.sort_by_key(|x| x.1);

        let mut code = 0;
        let mut prev_width = 0;
        for (symbol, bitwidth) in symbols {
            code <<= bitwidth - prev_width;
            self.set_mapping(symbol, Code::new(bitwidth, code));
            code += 1;
            prev_width = bitwidth;
        }
        self.finish()
    }
}

pub struct DecoderBuilder {
    table: Vec<u16>,
    eob_symbol: Option<u16>,
    eob_bitwidth: u8,
    max_bitwidth: u8,
}
impl DecoderBuilder {
    pub fn new(max_bitwidth: u8, eob_symbol: Option<u16>) -> Self {
        debug_assert!(max_bitwidth <= MAX_BITWIDTH);
        DecoderBuilder {
            table: vec![u16::from(MAX_BITWIDTH) + 1; 1 << max_bitwidth],
            eob_symbol: eob_symbol,
            eob_bitwidth: max_bitwidth,
            max_bitwidth: max_bitwidth,
        }
    }
    pub fn from_bitwidthes(bitwidthes: &[u8], eob_symbol: Option<u16>) -> Decoder {
        let builder = Self::new(bitwidthes.iter().cloned().max().unwrap_or(0), eob_symbol);
        builder.restore_canonical_huffman_codes(bitwidthes)
    }
}
impl Builder for DecoderBuilder {
    type Instance = Decoder;
    fn set_mapping(&mut self, symbol: u16, code: Code) {
        debug_assert!(code.width <= self.max_bitwidth);
        if Some(symbol) == self.eob_symbol {
            self.eob_bitwidth = code.width;
        }

        // `bitwidth` encoded `to` value
        let value = (symbol << 5) | u16::from(code.width);

        // Sets the mapping to all possible indices
        let code_be = code.inverse_endian();
        for padding in 0..(1 << (self.max_bitwidth - code.width)) {
            let i = ((padding << code.width) | code_be.bits) as usize;
            debug_assert_eq!(self.table[i], u16::from(MAX_BITWIDTH) + 1);
            unsafe {
                *self.table.get_unchecked_mut(i) = value;
            }
        }
    }
    fn finish(self) -> Self::Instance {
        Decoder {
            table: self.table,
            eob_bitwidth: self.eob_bitwidth,
            max_bitwidth: self.max_bitwidth,
        }
    }
}

#[derive(Debug)]
pub struct Decoder {
    table: Vec<u16>,
    eob_bitwidth: u8,
    max_bitwidth: u8,
}
impl Decoder {
    #[inline(always)]
    pub fn decode<R>(&self, reader: &mut bit::BitReader<R>) -> io::Result<u16>
    where
        R: io::Read,
    {
        let v = self.decode_unchecked(reader);
        reader.check_last_error()?;
        Ok(v)
    }

    #[inline(always)]
    pub fn decode_unchecked<R>(&self, reader: &mut bit::BitReader<R>) -> u16
    where
        R: io::Read,
    {
        let code = reader.peek_bits_unchecked(self.eob_bitwidth);
        let mut value = unsafe { *self.table.get_unchecked(code as usize) };
        let mut bitwidth = (value & 0b1_1111) as u8;
        if bitwidth > self.eob_bitwidth {
            let code = reader.peek_bits_unchecked(self.max_bitwidth);
            value = unsafe { *self.table.get_unchecked(code as usize) };
            bitwidth = (value & 0b1_1111) as u8;
            if bitwidth > self.max_bitwidth {
                reader.set_last_error(invalid_data_error!("Invalid huffman coded stream"));
            }
        }
        reader.skip_bits(bitwidth as u8);
        value >> 5
    }
}

#[derive(Debug)]
pub struct EncoderBuilder {
    table: Vec<Code>,
}
impl EncoderBuilder {
    pub fn new(symbol_count: usize) -> Self {
        EncoderBuilder {
            table: vec![Code::new(0, 0); symbol_count],
        }
    }
    pub fn from_bitwidthes(bitwidthes: &[u8]) -> Encoder {
        let symbol_count = bitwidthes
            .iter()
            .enumerate()
            .filter(|e| *e.1 > 0)
            .last()
            .map_or(0, |e| e.0) + 1;
        let builder = Self::new(symbol_count);
        builder.restore_canonical_huffman_codes(bitwidthes)
    }
    pub fn from_frequencies(symbol_frequencies: &[usize], max_bitwidth: u8) -> Encoder {
        let max_bitwidth = cmp::min(
            max_bitwidth,
            ordinary_huffman_codes::calc_optimal_max_bitwidth(symbol_frequencies),
        );
        let code_bitwidthes = length_limited_huffman_codes::calc(max_bitwidth, symbol_frequencies);
        Self::from_bitwidthes(&code_bitwidthes)
    }
}
impl Builder for EncoderBuilder {
    type Instance = Encoder;
    fn set_mapping(&mut self, symbol: u16, code: Code) {
        debug_assert_eq!(self.table[symbol as usize], Code::new(0, 0));
        self.table[symbol as usize] = code.inverse_endian();
    }
    fn finish(self) -> Self::Instance {
        Encoder { table: self.table }
    }
}

#[derive(Debug, Clone)]
pub struct Encoder {
    table: Vec<Code>,
}
impl Encoder {
    #[inline(always)]
    pub fn encode<W>(&self, writer: &mut bit::BitWriter<W>, symbol: u16) -> io::Result<()>
    where
        W: io::Write,
    {
        let code = self.lookup(symbol);
        debug_assert_ne!(code, Code::new(0, 0));
        writer.write_bits(code.width, code.bits)
    }
    #[inline(always)]
    pub fn lookup(&self, symbol: u16) -> Code {
        debug_assert!(
            symbol < self.table.len() as u16,
            "symbol:{}, table:{}",
            symbol,
            self.table.len()
        );
        unsafe { self.table.get_unchecked(symbol as usize) }.clone()
    }
    pub fn used_max_symbol(&self) -> Option<u16> {
        self.table
            .iter()
            .rev()
            .position(|x| x.width > 0)
            .map(|trailing_zeros| (self.table.len() - 1 - trailing_zeros) as u16)
    }
}

#[allow(dead_code)]
mod ordinary_huffman_codes {
    use std::cmp;
    use std::collections::BinaryHeap;

    pub fn calc_optimal_max_bitwidth(frequencies: &[usize]) -> u8 {
        let mut heap = BinaryHeap::new();
        for &freq in frequencies.iter().filter(|&&f| f > 0) {
            let weight = -(freq as isize);
            heap.push((weight, 0 as u8));
        }
        while heap.len() > 1 {
            let (weight1, width1) = heap.pop().unwrap();
            let (weight2, width2) = heap.pop().unwrap();
            heap.push((weight1 + weight2, 1 + cmp::max(width1, width2)));
        }
        let max_bitwidth = heap.pop().map_or(0, |x| x.1);
        cmp::max(1, max_bitwidth)
    }
}
mod length_limited_huffman_codes {
    use std::mem;

    #[derive(Debug, Clone)]
    struct Node {
        symbols: Vec<u16>,
        weight: usize,
    }
    impl Node {
        pub fn empty() -> Self {
            Node {
                symbols: vec![],
                weight: 0,
            }
        }
        pub fn single(symbol: u16, weight: usize) -> Self {
            Node {
                symbols: vec![symbol],
                weight: weight,
            }
        }
        pub fn merge(&mut self, other: Self) {
            self.weight += other.weight;
            self.symbols.extend(other.symbols);
        }
    }

    /// Reference: [A Fast Algorithm for Optimal Length-Limited Huffman Codes][LenLimHuff.pdf]
    ///
    /// [LenLimHuff.pdf]: https://www.ics.uci.edu/~dan/pubs/LenLimHuff.pdf
    pub fn calc(max_bitwidth: u8, frequencies: &[usize]) -> Vec<u8> {
        // NOTE: unoptimized implementation
        let mut source = frequencies
            .iter()
            .enumerate()
            .filter(|&(_, &f)| f > 0)
            .map(|(symbol, &weight)| Node::single(symbol as u16, weight))
            .collect::<Vec<_>>();
        source.sort_by_key(|o| o.weight);

        let weighted =
            (0..max_bitwidth - 1).fold(source.clone(), |w, _| merge(package(w), source.clone()));

        let mut code_bitwidthes = vec![0; frequencies.len()];
        for symbol in package(weighted)
            .into_iter()
            .flat_map(|n| n.symbols.into_iter())
        {
            code_bitwidthes[symbol as usize] += 1;
        }
        code_bitwidthes
    }
    fn merge(x: Vec<Node>, y: Vec<Node>) -> Vec<Node> {
        let mut z = Vec::with_capacity(x.len() + y.len());
        let mut x = x.into_iter().peekable();
        let mut y = y.into_iter().peekable();
        loop {
            let x_weight = x.peek().map(|s| s.weight);
            let y_weight = y.peek().map(|s| s.weight);
            if x_weight.is_none() {
                z.extend(y);
                break;
            } else if y_weight.is_none() {
                z.extend(x);
                break;
            } else if x_weight < y_weight {
                z.push(x.next().unwrap());
            } else {
                z.push(y.next().unwrap());
            }
        }
        z
    }
    fn package(mut nodes: Vec<Node>) -> Vec<Node> {
        if nodes.len() >= 2 {
            let new_len = nodes.len() / 2;

            #[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
            for i in 0..new_len {
                nodes[i] = mem::replace(&mut nodes[i * 2], Node::empty());
                let other = mem::replace(&mut nodes[i * 2 + 1], Node::empty());
                nodes[i].merge(other);
            }
            nodes.truncate(new_len);
        }
        nodes
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
