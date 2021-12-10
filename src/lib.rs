pub mod msg;
mod state;

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::{entry_point, to_binary};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    use cw721_metadata_onchain::Cw721MetadataContract;

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
        let cfg = Configuration::from_msg(&msg);

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
            ExecuteMsg::StubMsg {
                token_id: _,
                token_uri: _,
                owner_id: _,
                attributes: _,
            } => Ok(Response::default()),
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
            QueryMsg::NftInfo { token_id } if token_id == "stub" => to_binary(&Configuration::get_static_token(deps.storage)?),
            _ => Cw721MetadataContract::default().query(deps, env, msg),
        }
    }
}
