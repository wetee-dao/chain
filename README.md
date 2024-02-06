# WeTEE


<img align="right" width="400" src="https://raw.githubusercontent.com/w3f/Grants-Program/00855ef70bc503433dc9fccc057c2f66a426a82b/static/img/badge_black.svg" />

WeTEE is a decentralized application deployment platform integrated with a Trusted Execution Environment (TEE).

WeTEE consists of blockchain networks and multiple confidential computing clusters, collectively providing an efficient decentralised solution for confidential computing.

This project is funded by [Web3 Foundation](https://web3.foundation) via their [Open Grants Program](https://github.com/w3f/Open-Grants-Program)

## Introduction

WeTEE is a decentralized application deployment platform integrated with a Trusted Execution Environment (TEE).  

WeTEE consists of blockchain networks and multiple confidential computing clusters, collectively providing an efficient decentralised solution for confidential computing.  

We provide pallets to make it easier for create a DAO based on substrate.
- As a user, you can create any number of daos for yourself based on the chain.
- As a developer, you can quickly integrate DAOs into current projects

We provide the following pallets:  

> Click on the pallet name to view the api 

- [wetee-org](./packages/pallets/wetee-org/README.md) pallet  
    The basic DAO module through which you can create a DAO.  

- [wetee-assets](./packages/pallets/wetee-assets/README.md) pallet  
    The TOKEN management module specially designed for DAO, through which it can manage the native tokens of DAO on the chain and issue the organization tokens.  

- [wetee-sudo](./packages/pallets/wetee-sudo/README.md) pallet  
    For the early DAO, which requires the core team to quickly adjust various parameters. After the organization is stable, this module will be disabled.  

- [wetee-gov](./packages/pallets/wetee-gov/README.md) pallet  
    The governance module specially designed for DAO. Through this module, DAO members can conduct global voting, intra-guild voting and intra-project voting to achieve the purpose of intra-organization governance.  

- [wetee-guild](./packages/pallets/wetee-guild/README.md) pallet  
    This pallet is designed to organize the internal talent pool, and each guild will gather different groups of people.  

- [wetee-project](./packages/pallets/wetee-project/README.md) pallet  
    This pallet allows the DAO to run multiple projects at the same time, and each project team has multiple members from various guilds.  

## Setup local enviroment

### Rust Setup

- [Linux development environment](https://docs.substrate.io/install/linux/).  

- [MacOS development environment](https://docs.substrate.io/install/macos/).  

- [Windows development environment](https://docs.substrate.io/install/windows/).  

### Run Node
If you want to started with the example node, please refer to [run node](./docs/run-node.md) .  


### Run Docker
If you want to test the deploy enviroment, please refer to [run docker](./docs/run-docker.md) .  

