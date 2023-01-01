// web3stuff.rs - author: steinkirch
// methods for wallets and accounts

use std::str::FromStr;
use web3::types::{H160};

use crate::helpers::wei_to_eth;

pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;


pub async fn web3_connect(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting to {:?}", provider_url);
    let transport = web3::transports::Http::new(provider_url)?;
    
    get_accounts(web3::transports::Either::Right(transport), addresses).await
    
}


async fn get_accounts(transport: Transport, addresses: &str) -> web3::Result {

    let web3s = web3::Web3::new(transport);
    println!("✅ Tranport info: {:?}", web3s);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(addresses).unwrap());
    println!("✅ retrieving balances:");

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }

    Ok(())
}

