use std::str::FromStr;
use async_std::task;
use web3::futures::Future;

use web3::{
    ethabi::ethereum_types::U256,
    types::{Address, TransactionRequest},
};

/// Below sends a transaction to a local node that stores private keys (eg Ganache)
/// For generating and signing a transaction offline, before transmitting it to a public node (eg Infura) see transaction_public
async fn call_web3() -> web3::Result<()> {

    let transport = web3::transports::Http::new("HTTP://127.0.0.1:8545").unwrap();
    let web3 = web3::Web3::new(transport);


    // Insert the 20-byte "from" address in hex format (prefix with 0x)
    let from = Address::from_str("0xb884FF4b46C7d0cCb68d280EF39B3cBE8Fd47590").unwrap();

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x7b953144Da42fe99d15411ae9e945C877b1B839c").unwrap();

    // Build the tx object
    let tx_object = TransactionRequest {
        from,
        to: Some(to),
        value: Some(U256::exp10(17)), //0.1 eth
        ..Default::default()
    };

    // Send the tx to localhost
    let result = web3.eth().send_transaction(tx_object).await?;

    println!("Tx succeeded with hash: {}", result);

    Ok(())
}
#[tokio::main]
async fn main() {

    #[no_mangle]
    pub extern fn mob_web3() -> () {
        task::block_on(call_web3());
    }
}