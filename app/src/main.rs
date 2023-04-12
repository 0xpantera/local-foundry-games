use bindings::game_1::Game1;
use bindings::game_2::Game2;
use bindings::game_3::Game3;
use bindings::game_4::Game4;
use bindings::game_5::Game5;

use ethers::types::{Address, U256};
use ethers::providers::{Provider, Http, Middleware};

use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let sender: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;
    
    //call_game1(rpc_url, sender).await?;
    //call_game2(rpc_url, sender).await?;
    //call_game3(rpc_url, sender).await?;
    //call_game4(rpc_url, sender).await?;
    call_game5(rpc_url, sender).await?;

    Ok(())
}

async fn call_game5(rpc_url: &str, sender: Address) -> Result<()> {
    let contract_address: Address = "0x4ed7c70f96b99c776995fb64377f0d4ab3b0e1c1".parse()?;

    let prov = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(prov.with_sender(sender));

    let contract = Game5::new(contract_address, provider.clone());

    let amount: U256 = 10000.into();
    let allowance_call = contract.give_me_allowance(amount);
    let allowance_tx = allowance_call
        .send().await?
        .log_msg("sending allowance").await?.unwrap();

    let mint_call = contract.mint(amount);
    let mint_tx = mint_call
        .send().await?
        .log_msg("minting").await?.unwrap();

    let win_call = contract.win();
    let win_tx = win_call
        .send().await?
        .log_msg("winning").await?.unwrap();

    println!("RECEIPT:\n{:?}", win_tx);

    Ok(())
}

async fn call_game4(rpc_url: &str, sender: Address) -> Result<()> {
    let contract_address: Address = "0xc6e7df5e7b4f2a278906862b61205850344d4e7d".parse()?;

    let prov = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(prov.with_sender(sender));

    let contract = Game4::new(contract_address, provider.clone());
    
    let y: u8 = 210;
    let x: u8 = 56;

    let call_win = contract.win(x);
    let win_tx = call_win.send().await?.log_msg("winning...").await?.unwrap();
    println!("RECEIPT:\n{:?}", win_tx);

    Ok(())
}

async fn call_game3(rpc_url: &str, sender: Address) -> Result<()> {
    let contract_address: Address = "0x9a676e781a523b5d0c0e43731313a708cb607508".parse()?;

    let prov = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(prov.with_sender(sender));

    let contract = Game3::new(contract_address, provider.clone());
    
    // TODO: learn how to read public methods from contract
    // y: u8 should be read from cotract (`uint y = 210;`)
    let y = 210_u8;
    let threshold = 255_u8;
    let x = threshold - y;

    let call_win = contract.win(x);
    let win_tx = call_win.send().await?.log_msg("winning...").await?.unwrap();
    println!("RECEIPT:\n{:?}", win_tx);

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
