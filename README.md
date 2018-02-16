<div align="center">
  <img class="center" width="120" height="118" src="https://raw.githubusercontent.com/arshubham/srtnr/master/data/images/com.github.arshubham.srtnr.png" alt="Application Icon">
  <h1 align="center">Srtnr</h1>
  <h3 align="center">A simple tool to shorten long URLs!.</h3>
  <h4 align="center">Designed for <a href="https://elementary.io">elementary OS</h4>
  <a href="https://appcenter.elementary.io/com.github.arshubham.srtnr" target="_blank">
    <img align="center" src="https://appcenter.elementary.io/badge.svg" alt="Get it on AppCenter">
    </a>
    #####*Not yet available on appcenter
</div>

<br/>


<p align="center">
    <img src="https://raw.githubusercontent.com/arshubham/srtnr/master/data/images/Screenshot.png" alt="Screenshot"> <br>
</p>


## Building, Testing, Installation


### Requiered Dependencies

```
- elementary-sdk
- libnotify-dev
- pkgconf
- libssl-dev
- rustc
- cargo
- libpng-dev
- meson
```

### Delete Vendor Files 
##### (These are only needed for houston, remove them if you want to update the libraries and generate them again)

```
cd srtnr
rm -r vendor
rm -r .cargo
```

### Generate Vendor Files
```
cd srtnr
cargo install cargo-vendor    //Only need to install this once
make vendor
```

### Executing without Installation

```
cd srtnr
cargo run
```

### Building and Installation
#### (Build will use vendored crates if vendor folder is present, remove it to fetch crates from https://crates.io)

```
git clone https://github.com/arshubham/srtnr.git && cd srtnr
make clean
make
sudo make install
```

### Executing
```
com.github.arshubham.srtnr
```

### Uninstallation
```
cd srtnr
sudo make uninstall
```

## License

**_Srtnr_** is licensed under GNU GPL v3.0
