prefix ?= /usr
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin
includedir = $(prefix)/include
datarootdir = $(prefix)/share
datadir = $(datarootdir)

.PHONY: all install uninstall

BIN=com.github.arshubham.srtnr

SRC=src/* src/*/*

all: target/release/$(BIN)

debug: target/debug/$(BIN)

clean:
	cargo clean

install: all
	install -D -m 0755 "target/release/$(BIN)" "$(DESTDIR)$(bindir)/$(BIN)"

uninstall: rm -f "$(DESTDIR)$(bindir)/$(BIN)"

target/release/$(BIN): $(SRC)
		cargo build --release -p srtnr && mv target/release/srtnr target/release/$(BIN)