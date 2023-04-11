use bindings::game_1::Game1;
use bindings::game_2::Game2;

use ethers::{types::Address};
use ethers::providers::{Provider, Http, Middleware};

use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let sender: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;
    
    call_game1(rpc_url, sender).await?;
    call_game2(rpc_url, sender).await?;

    Ok(())
}

async fn call_game2(rpc_url: &str, sender: Address) -> Result<()> {
    let contract_address: Address = "0x8a791620dd6260079bf849dc5567adc3f2fdc318".parse()?;

    let prov = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(prov.with_sender(sender));

    let contract = Game2::new(contract_address, provider.clone());

    let call_setx = contract.set_x(30.into());
    let setx_tx = call_setx.send().await?.log_msg("Setting x");

    let call_sety = contract.set_y(20.into());
    let sety_tx = call_sety.send().await?.log_msg("Setting y");

    let call_win = contract.win();
    let win_tx = call_win.send().await?;
    println!("RECEIPT:\n {:?}", win_tx);


    Ok(())
}

async fn call_game1(rpc_url: &str, sender: Address) -> Result<()> {
    let contract_address: Address = "0xa513e6e4b8f2a923d98304ec87f64353c4d5c853".parse()?;

    let prov = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(prov.with_sender(sender));

    println!("SENDER ADDRESS: \n{:?}", provider.default_sender().unwrap());
    let contract = Game1::new(contract_address, provider.clone());
    println!("CONTRACT ADDRESS: \n{}", contract.address());

    let call = contract.win();
    let pending_tx = call.send().await?;

    let receipt = pending_tx.await?;
    println!("RECEIPT:\n {:?}", receipt.unwrap());

    Ok(())
}
