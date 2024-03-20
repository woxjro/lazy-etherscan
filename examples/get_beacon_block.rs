use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::primitives::Root;
use ethers::utils::hex;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("http://localhost:5052").unwrap();
    let client = Client::new(url);

    let root_hex =
        hex::decode("421c16805c3416150aa04533fdfe7fc3f0880d0ed86cee33fa58011f10dd95c8").unwrap();
    let _root = Root::try_from(root_hex.as_ref()).unwrap();
    let id = BlockId::Finalized;

    let block = client.get_beacon_block(id).await.unwrap();
    dbg!(block);
}
