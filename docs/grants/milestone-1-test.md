# Milestone 1 Test Guide
  
### Rust Setup

- [Linux development environment](https://docs.substrate.io/install/linux/).
- [MacOS development environment](https://docs.substrate.io/install/macos/).
- [Windows development environment](https://docs.substrate.io/install/windows/).


### Test all pallets

``` bash
cargo test -- --nocapture
```

### Test Worker pallet

``` bash
cargo test -p wetee-worker -- --nocapture
```

### Test Tee App pallet

``` bash
cargo test -p wetee-app -- --nocapture
```

### Test Tee Task pallet

``` bash
cargo test -p wetee-task -- --nocapture
```
