# Milestone 1 Test Guide
  
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
