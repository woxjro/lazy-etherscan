//use ethers_core::types::Address;
use ethers_core::types::BlockNumber;
use ethers_providers::{Http, Middleware, Provider};
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;

    let block_number = provider.get_block_number().await?;
    println!("{}", block_number);

    let finalized_block = provider.get_block_with_txs(BlockNumber::Finalized).await?;
    let safe_block = provider.get_block(BlockNumber::Safe).await?;

    //let addr = "0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359".parse::<Address>()?;
    //let code = provider.get_code(addr, None).await?;
    let transactions = finalized_block.unwrap().transactions;

    for tx in transactions {
        println!("{:?}", tx.value);
    }

    //println!("{:?}", safe_block.unwrap().number.unwrap());
    Ok(())
}
