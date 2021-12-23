pub mod msg;
mod state;

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

    pub use crate::msg::{ExecuteMsg, InstantiateMsg, MsgMap};
    use crate::state::Configuration;

    type Cw721InstantiateMsg = cw721_base::InstantiateMsg;
    type Cw721ExecuteMsg = cw721_metadata_onchain::ExecuteMsg;

    /// This impl should probably go somewhere else but I don't fully understand managing scope for
    /// trait implementations.
    impl MsgMap for Cw721InstantiateMsg {
        fn from_wrapper(msg: InstantiateMsg) -> Cw721InstantiateMsg {
            Cw721InstantiateMsg {
                name: msg.name,
                symbol: msg.symbol,
                minter: msg.minter,
            }
        }
    }

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let cfg = Configuration::from_msg::<Metadata>(&msg);

        cfg.store(deps.api, deps.storage)?;

        let core_msg = Cw721InstantiateMsg::from_wrapper(msg);
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
                Cw721ExecuteMsg::from(msg),
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

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cosmwasm_std::testing::{
        mock_env as mock_env_std, MockApi as MockApi_std, MockQuerier as MockQuerier_std,
        MockStorage as MockStorage_std,
    };

    use terra_multi_test::{App, BankKeeper, ContractWrapper, Executor, TerraMockQuerier};

    use cw721::{NftInfoResponse, Cw721QueryMsg};
    use cw721_metadata_onchain::Metadata;

    use crate::entry::InstantiateMsg;

    /// Lifted from astropost
    /// https://github.com/astroport-fi/astroport-lbport/blob/ee24a0c532ec01a8af61ef58d5efc689bded1a16/contracts/factory/tests/integration.rs#L13
    fn mock_app() -> App {
        let env = mock_env_std();
        let api = MockApi_std::default();
        let bank = BankKeeper::new();
        let storage = MockStorage_std::new();
        let terra_mock_querier = TerraMockQuerier::new(MockQuerier_std::new(&[]));

        App::new(api, env.block, bank, storage, terra_mock_querier)
    }

    #[test]
    fn stub_with_query_wasm_smart() {
        let mut app = mock_app();
        
        let contract = Box::new(
            ContractWrapper::new(
                crate::entry::execute,
                crate::entry::instantiate,
                crate::entry::query,
            )
        );

        let code_id = app.store_code(contract);

        let owner = "owner1";

        let msg = InstantiateMsg {
            always_owner: None,
            minter: owner.to_string(),
            name: "test1".to_string(),
            symbol: "TEST1".to_string(),
            static_token: None,
        };
        
        // instantiate
        let factory_instance = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("admin1"),
                &msg,
                &[],
                "TerraStubNft",
                None,
            )
            .unwrap();

        let nft_info = Cw721QueryMsg::NftInfo {
            token_id: "stub".to_string(),
        };
            

        // test that query_wasm_smart will succesfully return an NftInfoResponse
        let info: NftInfoResponse<Metadata> = app.wrap().query_wasm_smart(
                factory_instance,
                &nft_info).unwrap();

        assert_eq!(Some("https://stub.test/stub_token.json".to_string()), info.token_uri);
    }
}
