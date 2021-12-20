use anyhow::Error;
use terra_rust_api::{Terra, GasOptions, PrivateKey};
use terra_rust_api::core_types::{Coin, StdSignMsg, StdSignature};
use terra_rust_api::messages::{MsgSend, Message};
use bitcoin::secp256k1::Secp256k1;

fn main() {
    println!("Hello, contract!");

    let result = doTheThings();

    match result {
        Ok(msg) =>  println!("All done! ({}", msg),
        Err(err) => println!("No fun: {}", err)
    }
}

fn doTheThings() -> Result<String, Error> {
    // To start, just give relative file path
    // Initialize client
    let gas_opts = GasOptions::create_with_gas_estimate("50ukrw",1.4)?;
    let terra = Terra::lcd_client("https://localhost:1417/", "localterra", &gas_opts,None);

    // My features
    // 1. Build message
    // 2. Send
    // 3. Verify all good
    //
    Ok("Moar gud".to_string())
}
