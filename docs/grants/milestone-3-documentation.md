# Milestone 2 Documentation

## Hardware and Software Requirements

- [CPU List - click to see cpu list](https://ark.intel.com/content/www/us/en/ark/search/featurefilter.html?productType=873&2_SoftwareGuardExtensions=Yes%20with%20Intel%C2%AE%20ME)
  - Intel 8th generation (Cannon Lake) Core i3, i5, i7, and i9 processors
  - Intel 9th generation (Cascade Lake) Core i3, i5, i7, and i9 processors
  - Intel 10th generation (Comet Lake) Core i3, i5, i7, and i9 processors
  - 2nd Generation Xeon Scalable processors (Cascade Lake) and later generations generally provide SGX capabilities.
- OS ubuntu 20.04 or ubuntu 22.04 (not in docker)

## Setup local enviroment

### Intel Sgx Setup on Ubuntu 20.04/Ubuntu 22.04 and Ego Setup

> For more information about Ego, please refer to https://docs.edgeless.systems/ego/getting-started/install

```bash
sudo apt install build-essential libssl-dev

sudo mkdir -p /etc/apt/keyrings
wget -qO- https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | sudo tee /etc/apt/keyrings/intel-sgx-keyring.asc > /dev/null
echo "deb [signed-by=/etc/apt/keyrings/intel-sgx-keyring.asc arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/intel-sgx.list
sudo apt update

EGO_DEB=ego_1.4.1_amd64_ubuntu-$(lsb_release -rs).deb
wget https://github.com/edgelesssys/ego/releases/download/v1.4.1/$EGO_DEB
sudo apt install ./$EGO_DEB build-essential libssl-dev

sudo mkdir /opt/wetee-worker
sudo chmod 777 /opt/wetee-worker
```

### K3s Setup

> For more information about K3s, please refer to https://docs.k3s.io/quick-start

```bash
curl -sfL https://get.k3s.io | sh -
```

### Set golang env

```bash
# Install golang 1.20 ,ubuntu 20.04 default golang version is 1.13, is too low
sudo apt install golang-1.21

# set golang env, also you can add it to .bashrc file（in home dir) or .zshrc file（in home dir)
export GOROOT=/usr/lib/go-1.21/
export PATH=$PATH:$GOROOT/bin
```

## Run worker images

```bash
git clone  https://github.com/wetee-dao/worker && cd worker

# 1.0 Setup Env
go mod tidy
sudo chmod 744 /etc/rancher/k3s/k3s.yaml

# 1.1 install addn docker images to k3s
sh hack/pre_install.sh

# Wait until all the images above have been installed, and use `kubectl get pod -A` to check whether sgx-device-plugin-, sgx-pccs-api-, wetee-dapp-*, and wetee-node-* have been successfully deployed.

# 1.2 and then install worker
sh hack/install.sh
```

## 1.As cluster start quick mint (The normal mining process has been validated in M2.)

- Open cluster worker address `http:// {server IP} :30000/`
- Input data and signature like this
  ```
  mutation{
    start_for_test
  }
  ```
- Click `Execute` button and check result has no error
  <img src="./img/m3/1.png" width="700">

## 2. Login to DAPP

- Open Dapp address `http:// {server IP} :30002/`
- Select `Account` and click `Login` button
  <img src="./img/m3/2.png" width="700">

## 3 Send token to test account

- Open polkadot UI http://polkadot-ui.tc.asyou.me/#/accounts
- Connect to `development node` --> `Custom node` ->
  `ws://{server IP}:30001`
  <img src="./img/m3/3.png" width="700">
- Open `Accounts` --> `Transfer` section
- Input `{MintAddress}` to `send to address`
- Input `100000` to `amount`
- Click `Transfer` button and check result has no error

### 1.3 Start a TEE service

- Open Dapp address `http://{server IP}:30002/`
- Open `Personal Project`
  <img src="./img/m3/4.png" width="700">

- Click `New button` --> `Confidential Service`
  <img src="./img/m3/5.png" width="700">
- Click `Add button` sign and submit to chain

### 1.4 Check metrics of the new service

- Click `Application card` -> click `Metrics` tab
  <img src="./img/m3/8.png" width="700">
- Open polkadot UI http://polkadot-ui.tc.asyou.me
- Open `Developer` --> `Chain state` section
- Select `weteeWorker` --> `proofsOfWork` callable
- Query like this
  <img src="./img/m3/7.png" width="700">
- All metrics and data hash are displayed in the `result` field

### 1.4 Check logs of the new service

- Click `Application card` -> click `Log` tab
  <img src="./img/m3/6.png" width="700">
- Open polkadot UI http://polkadot-ui.tc.asyou.me
- Open `Developer` --> `Chain state` section
- Select `weteeWorker` --> `proofsOfWork` callable
- Query like this
  <img src="./img/m3/7.png" width="700">
- All logs hash are displayed in the `result` field

### 1.5 Check  remote attestation  of the new service

- Click `Application card` -> click `Sgx report` tab
  <img src="./img/m3/9.png" width="700">
- Open polkadot UI http://polkadot-ui.tc.asyou.me
- Open `Developer` --> `Chain state` section
- Select `weteeWorker` --> `reportOfWork` callable
- Query like this
  <img src="./img/m3/10.png" width="700">
- All logs hash are displayed in the `result` field
