use anyhow::Error;
use bitcoin::secp256k1::Secp256k1;
use std::path::Path;
use terra_rust_api::messages::wasm::MsgStoreCode;
use terra_rust_api::messages::Message;
use terra_rust_api::{GasOptions, PrivateKey, Terra};

#[tokio::main]
async fn main() {
    println!("Hello, contract!");

    let result = do_the_things().await;

    match result {
        Ok(msg) => println!("All done! ({}", msg),
        Err(err) => println!("No fun: {}", err),
    }
}

async fn do_the_things() -> Result<String, Error> {
    // To start, just give relative file path
    // Initialize client
    let gas_opts = GasOptions::create_with_gas_estimate("50ukrw", 1.4)?;
    let terra = Terra::lcd_client("https://localhost:1317/", "localterra", &gas_opts, None);

    let secp = Secp256k1::new();
    let from_key = PrivateKey::from_words(&secp,"notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius",0,0)?;

    // test1 on localterra
    let sender = "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";
    let wasm_file = Path::new("../artifacts/terra_nft_stub.wasm");

    let msg = MsgStoreCode::create_from_file(sender, wasm_file)?;

    let messages: Vec<Message> = vec![msg];
    let (std_sign_msg, sigs) = terra
        .generate_transaction_to_broadcast(&secp, &from_key, messages, None)
        .await?;

    // send it out
    let resp = terra.tx().broadcast_sync(&std_sign_msg, &sigs).await?;

    match resp.code {
        Some(code) => {
            log::error!("{}", serde_json::to_string(&resp)?);
            eprintln!("Transaction returned a {} {}", code, resp.txhash)
        }
        None => {
            println!("{}", resp.txhash)
        }
    }
    // My features
    // 1. Build message
    // 2. Send
    // 3. Verify all good
    //
    Ok("Moar gud".to_string())
}
