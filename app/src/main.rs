use bindings::game_1::Game1;

use ethers::{types::Address};
use ethers::providers::{Provider, Http, Middleware};

use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let address: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;
    let contract_address: Address = "0xa513e6e4b8f2a923d98304ec87f64353c4d5c853".parse()?;

    let prov = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(prov.with_sender(address));

    println!("SENDER ADDRESS: \n{:?}", provider.default_sender().unwrap());
    let contract = Game1::new(contract_address, provider.clone());
    println!("CONTRACT ADDRESS: \n{}", contract.address());

    let call = contract.win();
    let pending_tx = call.send().await?;

    let receipt = pending_tx.await?;
    println!("RECEIPT:\n {:?}", receipt.unwrap());

    Ok(())
}
