pub mod msg;
mod state;

mod integration_tests;

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::{entry_point, to_binary};
    use cosmwasm_std::{
        Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    };

    use cw721::{AllNftInfoResponse, OwnerOfResponse};
    use cw721_base::Cw721Contract;
    use cw721_metadata_onchain::{Cw721MetadataContract, Metadata};

    pub use cw721_base::state::TokenInfo;
    pub use cw721_base::{ContractError, MintMsg, MinterResponse, QueryMsg};

    pub use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::Configuration;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let cfg: Configuration = (&msg).into();

        cfg.store(deps.api, deps.storage)?;

        let core_msg = (&msg).into();
        Cw721MetadataContract::default().instantiate(deps, env, info, core_msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Stub {
                token_id,
                token_uri,
                owner_id,
                attributes,
            } => stub::<String>(deps, &attributes, &token_uri, &token_id, &owner_id),
            _ => Cw721MetadataContract::default().execute(
                deps,
                env,
                info,
                msg.into(),
            ),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        // This needs to handle static stub on NftInfo and AllNftInfo
        match msg {
            QueryMsg::NftInfo { token_id } if token_id == "stub" => {
                to_binary(&Configuration::get_static_token(deps.storage)?)
            }
            QueryMsg::AllNftInfo {
                include_expired: _,
                token_id,
            } if token_id == "stub" => {
                let info = Configuration::get_static_token(deps.storage)?;
                let access = OwnerOfResponse {
                    approvals: vec![],
                    owner: Configuration::get_owner(deps.storage)?.to_string(),
                };

                to_binary(&AllNftInfoResponse { access, info })
            }
            _ => Cw721MetadataContract::default().query(deps, env, msg),
        }
    }

    fn stub<C>(
        deps: DepsMut,
        attributes: &str,
        token_uri: &str,
        token_id: &str,
        owner_id: &str,
    ) -> Result<Response, ContractError> {
        if let Ok(ext) = serde_json_wasm::from_str(attributes) {
            let contract = Cw721Contract::<'_, Metadata, C>::default();

            if token_uri.is_empty() {
                return Err(ContractError::Std(StdError::generic_err(
                    "token_uri must not be empty".to_string(),
                )));
            }

            if token_id.is_empty() {
                return Err(ContractError::Std(StdError::generic_err(
                    "token_id must not be empty".to_string(),
                )));
            }

            let owner: Addr = deps.api.addr_validate(owner_id)?;

            let token: TokenInfo<Metadata> = TokenInfo {
                owner,
                approvals: vec![],
                token_uri: Some(token_uri.to_string()),
                extension: ext,
            };

            Configuration::claimed(deps.storage, token_uri)?;
            Configuration::store_token_by_uri::<Metadata>(deps.storage, token_uri)?;
            contract
                .tokens
                .update(deps.storage, token_id, |old| match old {
                    Some(_) => Err(ContractError::Claimed {}),
                    None => Ok(token.clone()),
                })?;

            contract.increment_tokens(deps.storage)?;

            let minter = contract.minter.load(deps.storage)?;

            Ok(Response::new()
                .add_attribute("action", "mint")
                .add_attribute("minter", minter)
                .add_attribute("token_id", token_id))
        } else {
            Err(ContractError::Std(StdError::generic_err(
                "Unable to deserialize attributes".to_string(),
            )))
        }
    }
}
