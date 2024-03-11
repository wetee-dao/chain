# Milestone 3 Test Guide

### Go Setup on Ubuntu 20.04/Ubuntu 22.04

```
sudo apt install golang-1.21
```

### Test all

```bash
git clone  https://github.com/wetee-dao/worker && cd worker/mint/proof
go test

git clone  https://github.com/wetee-dao/libos-entry && cd libos-entry
go test ./...
```
