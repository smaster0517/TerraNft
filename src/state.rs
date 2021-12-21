use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{Addr, Api, Response, StdError, StdResult, Storage};
use cw721::NftInfoResponse;
use cw721_base::ContractError;
use cw721_metadata_onchain::Metadata;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

use crate::msg::InstantiateMsg;

/// Lifted from TerraPeeps as an indexed data type witihin Terra (CW) storage.
pub struct TokenIndexString<'a> {
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, (String, Vec<u8>), String>,
}

impl<'a> IndexList<String> for TokenIndexString<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<String>> + '_> {
        let v: Vec<&dyn Index<String>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub struct Configuration {
    pub always_owner: Option<String>,
    pub static_token: Option<String>,
}

#[allow(clippy::ptr_arg)]
pub fn token_owner_idx_string(d: &String, k: Vec<u8>) -> (String, Vec<u8>) {
    (d.clone(), k)
}

impl Configuration {
    pub fn from_msg<T>(msg: &InstantiateMsg) -> Configuration
    where
        T: Serialize + DeserializeOwned + Clone,
    {
        Configuration {
            always_owner: msg.always_owner.clone(),
            static_token: msg.static_token.clone(),
        }
    }

    fn indexed_token_uris<'a>() -> IndexedMap<'a, &'a str, String, TokenIndexString<'a>> {
        let uri_indexes = TokenIndexString {
            owner: MultiIndex::new(token_owner_idx_string, "tokens_uri", "tokens_uri__owner"),
        };

        IndexedMap::new("tokens_uri", uri_indexes)
    }

    pub fn claimed<'a>(
        store: &mut dyn Storage,
        token_uri: &'a str,
    ) -> Result<&'a str, ContractError> {
        let tokens_store = Configuration::indexed_token_uris();

        if let Ok(_x) = tokens_store.load(store, token_uri) {
            return Err(ContractError::Claimed {});
        }

        Ok(token_uri)
    }

    pub fn store_token_by_uri<'a, T>(
        store: &mut dyn Storage,
        token_uri: &'a str,
    ) -> Result<&'a str, ContractError> {
        Configuration::indexed_token_uris().update(store, token_uri, |old| match old {
            Some(_) => Err(ContractError::Claimed {}),
            None => Ok(token_uri.to_string()),
        })?;

        Ok(token_uri)
    }

    pub fn store(&self, api: &dyn Api, store: &mut dyn Storage) -> StdResult<Response> {
        if let Some(ao) = &self.always_owner {
            let always_owner: Addr = api.addr_validate(ao)?;
            let storage = Item::new("always_owner");
            storage.save(store, &always_owner)?;
        }

        if let Some(st) = &self.static_token {
            if serde_json_wasm::from_str::<Metadata>(st).is_err() {
                return Err(StdError::SerializeErr {
                    source_type: "Metadata".to_string(),
                    msg: "Could not deserialize input string".to_string(),
                });
            }

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
        if let Ok(stub_str) = Item::<'_, String>::new("static_token").load(store) {
            let result = serde_json_wasm::from_str(&stub_str);

            if let Ok(extension) = result {
                return Ok(NftInfoResponse {
                    token_uri: None,
                    extension,
                });
            }
        }
        Err(StdError::SerializeErr {
            source_type: "Metadata".to_string(),
            msg: "Could not deserialize stored stub token".to_string(),
        })
    }

    pub fn get_owner(store: &dyn Storage) -> StdResult<Addr> {
        if Item::<'_, Addr>::new("always_owner").load(store).is_ok() { }

        Err(StdError::GenericErr {
            msg: "Unable to load always_owner Addr".to_string(),
        })
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
