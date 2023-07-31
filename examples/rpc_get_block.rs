//use ethers_core::types::Address;
use ethers_providers::{Http, Middleware, Provider};
use futures::future::join_all;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;

    let block_number = provider.get_block_number().await?;
    println!("{}", block_number);

    let mut blocks = vec![];
    for i in 0..10 {
        let block = provider.get_block(block_number - i);
        blocks.push(block);
    }

    let blocks = join_all(blocks).await;

    //let addr = "0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359".parse::<Address>()?;
    //let code = provider.get_code(addr, None).await?;
    for block in blocks {
        println!("{:?}", block.unwrap().unwrap().transactions.len());
    }
    Ok(())
}
