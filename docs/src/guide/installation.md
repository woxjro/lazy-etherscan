# Installation

This software has been tested and verified to work correctly on the following operating systems:
- `Ubuntu 22.04.2 LTS`
- `macOS Ventura 13.2`



## Prerequisites
### Optional: Etherscan API Key
To see statistics information about Ethereum, you have to set an Etherscan's free API key.
You can get it from [here](https://etherscan.io/apis).
And add it to your environment variables. If you are using `zsh`, run the following command.
```sh
$ echo 'export ETHERSCAN_API_KEY=XXXXXXXXXXXX' >> ~/.zshenv
```

### Optional: [`ethereum-input-data-decoder`](https://github.com/miguelmota/ethereum-input-data-decoder)
To see transactions' decoded input data, you have to preinstall [`ethereum-input-data-decoder`](https://github.com/miguelmota/ethereum-input-data-decoder). Please run the following command.
```sh
$ npm install -g ethereum-input-data-decoder
```


## Installation using Cargo
```sh
$ cargo install lazy-etherscan
$ lazy-etherscan
```

## Build from source
```sh
$ git clone https://github.com/woxjro/lazy-etherscan --recursive
$ cd lazy-etherscan
$ cargo run --
```
