use cosmwasm_std::{Addr, Api, Response, StdError, StdResult, Storage};
use cw721::NftInfoResponse;
use cw721_metadata_onchain::Metadata;
use cw_storage_plus::Item;

use crate::msg::InstantiateMsg;

pub struct Configuration {
    pub always_owner: Option<String>,
    pub static_token: Option<String>,
}

impl<'a> Configuration {
    pub fn from_msg(msg: &InstantiateMsg) -> Configuration {
        Configuration {
            always_owner: msg.always_owner.clone(),
            static_token: msg.static_token.clone(),
        }
    }

    pub fn store(&self, api: &dyn Api, store: &mut dyn Storage) -> StdResult<Response> {
        if let Some(ao) = &self.always_owner {
            let always_owner: Addr = api.addr_validate(ao)?;
            let storage = Item::new("always_owner");
            storage.save(store, &always_owner)?;
        }

        if let Some(st) = &self.static_token {
            // validate it is a Metadata?

            // store it
            let storage = Item::new("static_token");
            storage.save(store, st)?;
        } else {
            let data = Metadata::static_default();

            if let Ok(ser) = serde_json_wasm::to_string(&data) {
                let storage = Item::new("static_token");
                storage.save(store, &ser)?;
            } else {
                return Err(StdError::SerializeErr {
                    source_type: "Metadata".to_string(),
                    msg: "Could not store static token".to_string(),
                });
            }
        }

        Ok(Response::default())
    }

    pub fn get_static_token(store: &dyn Storage) -> StdResult<NftInfoResponse<Metadata>> {
        if let Ok(stub_str) = Item::<'a, String>::new("static_token").load(store) {
            let result = serde_json_wasm::from_str(&stub_str);

            if let Ok(extension) = result {
                return Ok(NftInfoResponse {
                            token_uri: None,
                            extension });
            }
        }
        Err(StdError::SerializeErr{ source_type: "Metadata".to_string(), msg: "Could not deserialize stored stub token".to_string()})
    }
}

trait StaticMetadata {
    fn static_default() -> Metadata;
}

impl StaticMetadata for Metadata {
    fn static_default() -> Metadata {
        Metadata {
            image: Some("https://d75aawrtvbfp1.cloudfront.net/ipfs://QmVFGFfntmSTM98bzCSGyXsnmdius9vSGy74r1KAwQbTxY".to_string()),
            image_data: Some(r#"<svg xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 50 50"><circle cx="25" cy="25" r="15" stroke="black" stroke-width="3" fill="red" /></svg> "#.to_string()),
            external_url: None,
            description: Some("The default stub NFT".to_string()),
            name: Some("Stubby".to_string()),
            attributes: Some(vec![]),
            background_color: None,
            animation_url: None,
            youtube_url: None,
        }
    }
}
