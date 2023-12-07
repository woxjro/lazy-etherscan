# Configuration

## Ethereum Statistics
To see statistics information about Ethereum, you have to set an Etherscan's free API key.
You can get it from [here](https://etherscan.io/apis).
And set it to `api_key` field in `settings.toml`. You can use a setting template file (`settings.example.toml`).
```sh
$ mv ./settings.example.toml ./settings.toml
```

## Endpoint
The default endpoint is https://eth.public-rpc.com, and you can also set your preferred endpoint.
You can find free endpoints from [ChainList](https://chainlist.org/chain/1).
To set your endpoint, run with a `--endpoint` option.
```sh
cargo run -- --endpoint=https://rpc.flashbots.net
```

In the case of the L2 blockchain networks and BSC RPC endpoints listed below,
it has been confirmed that this software works to some extent.

| Platform        | RPC Endpoint                                                                       |
| --------------- | ---------------------------------------------------------------------------------- |
| Arbitrum One    | [https://arb1.arbitrum.io/rpc](https://arb1.arbitrum.io/rpc)                       |
| Optimism        | [https://mainnet.optimism.io](https://mainnet.optimism.io)                         |
| Boba Network    | [https://lightning-replica.boba.network/](https://lightning-replica.boba.network/) |
| BNB Smart Chain | [https://bsc-dataseed.bnbchain.org](https://bsc-dataseed.bnbchain.org )            |

## Other Configuration
To check other configurations, run the following command.
```sh
cargo run -- --help
```
