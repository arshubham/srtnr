<div align="center">
  <img class="center" width="80" height="78" src="https://raw.githubusercontent.com/arshubham/srtnr/master/data/images/com.github.arshubham.srtnr.png" alt="Application Icon">
  <h1 align="center">Srtnr</h1>
  <h3 align="center">A simple tool to shorten long URLs!.</h3>
  <h4 align="center">Designed for <a href="https://elementary.io">elementary OS</h4>
  <a href="https://appcenter.elementary.io/com.github.arshubham.srtnr" target="_blank">
    <img align="center" src="https://appcenter.elementary.io/badge.svg" alt="Get it on AppCenter">
    </a>
</div>

<br/>


<p align="center">
    <img src="https://raw.githubusercontent.com/arshubham/srtnr/master/data/images/Screenshot.png" alt="Screenshot"> <br>
</p>


## Building, Testing, Installation


### Requiered Dependencies

```
- pkgconf
- libssl-dev
- rustc
- cargo
- meson
```

**Note** : Use rustc and cargo from repository instead of rustup for now. There are some issues when running ```ninja install``` . You can still use the **rustup** version if you just want to run the application using cargo.

### Executing without Installation

```
cd srtnr
cargo run
```

### Building and Installation

```
git clone https://github.com/arshubham/srtnr.git && cd srtnr
meson build && cd build
ninja
sudo ninja install

```

### Executing
```
com.github.arshubham.srtnr
```

## License

**_Srtnr_** is licensed under GNU GPL v3.0
