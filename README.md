# lazy-etherscan

<div align="left">
    <a href="https://github.com/woxjro/lazy-etherscan/"><img alt="Static Badge" src="https://img.shields.io/badge/github-woxjro%2Flazy-etherscan?style=for-the-badge&logo=github" height="20"></a>
    <a href="https://github.com/woxjro/lazy-etherscan/actions"><img alt="build status" src="https://img.shields.io/github/actions/workflow/status/woxjro/lazy-etherscan/rust.yml?style=for-the-badge" height="20"></a>
    <a href="https://github.com/woxjro/lazy-etherscan/blob/master/LICENSE"><img alt="GitHub" src="https://img.shields.io/github/license/woxjro/lazy-etherscan?style=for-the-badge" height="20"></a>
</div>

![demo](https://github.com/woxjro/lazy-etherscan/assets/63214188/da2d1662-1c49-433c-b32e-bb4460977a56)

<details>
 <summary><strong>Table of contents</strong></summary>
 <br/>

- [lazy-etherscan](#lazy-etherscan)
  - [Build](#build)
  - [Configurations & Usage](#configurations--usage)
  - [Roadmap](#roadmap)
  - [Contributing](#contributing)
  - [Acknowledgement](#acknowledgement)

<br/>
</details>


## Build
This software has been tested and verified to work correctly on the following operating systems:
- `Ubuntu 22.04.2 LTS`

```sh
$ git clone https://github.com/woxjro/lazy-etherscan --recursive
$ cd lazy-etherscan
$ cargo run --
```

## Configurations & Usage
Please check the various settings such as endpoints using the following command:
```sh
cargo run -- --help
```

The basic usage is as follows:
- Press `q` to exit `lazy-etherscan`.
- Press `s` to focus on the search bar, where you can perform searches for addresses, blocks, transactions, and more.
- Press `1` to navigate to the "Latest Blocks" panel. Use `j` to move to a block below and `k` to an above block.
- Press `2` to navigate to the "Latest Transactions" panel. Use `j` to move to a transaction below and `k` to move to a transaction above.


## Roadmap
In the current state of this project, I plan to implement the following functionalities:
- [ ] Welcome Page
    - [ ] Display statistics about Ethereum.
    - [ ] Latest Blocks
        - [x] Display a list of the latest blocks.
        - [ ] Display details of a block being focused.
    - [ ] Latest Transactions
        - [x] Display a list of the latest transactions.
        - [ ] Display details of a transaction being focused.
- [ ] Search Functionality
    - [ ] by Address
        - [ ] Implement search results screen.
    - [ ] by Txn Hash
        - [ ] Implement search results screen.
    - [ ] by Block
        - [ ] Implement search results screen.
    - [ ] by Token
        - [ ] Implement search results screen.
    - [ ] by Domain Name
        - [ ] Implement search results screen.


## Contributing
This project is currently in the Proof of Concept (PoC) stage.
Contributions are kindly requested to be postponed until further notice.


## Acknowledgement
`lazy-etherscan` is written in [Rust](https://www.rust-lang.org/) and is built on top of [ratatui](https://github.com/ratatui-org/ratatui).
This project is highly inspired by [Etherscan](https://etherscan.io/), [lazygit](https://github.com/jesseduffield/lazygit) and [spotify-tui](https://github.com/Rigellute/spotify-tui).
