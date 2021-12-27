use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Binary;

use cw721::Expiration;
use cw721_base::MintMsg;
use cw721_metadata_onchain::Extension;

type Cw721InstantiateMsg = cw721_base::InstantiateMsg;
type Cw721ExecuteMsg = cw721_metadata_onchain::ExecuteMsg;

/// This Msg duplicates the fields from cw721_base::InstantiateMsg and extends on those for the
/// purposes of this contract.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    /// (Cw721InstantiateMsg) The name of the token
    pub name: String,
    /// (Cw721InstantiateMsg) The symbol of the token
    pub symbol: String,
    /// (Cw721InstantiateMsg) The address that can mint new tokens
    pub minter: String,

    /// A wallet address as a String that will _always_ return the static token from nft_info and
    /// all_nft_info.
    pub always_owner: String,

    /// Specify a serialized full token to use as the static token returned from nft_info and
    /// all_nft_info. Note that a default static token will be set if this is not provided!
    pub static_token: Option<String>,
}

pub trait MsgMap {
    fn from_wrapper(msg: InstantiateMsg) -> Cw721InstantiateMsg;
}

/// Have to copy Msg variants from base, as
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Stub {
        token_id: String,
        token_uri: String,
        owner_id: String,
        attributes: String,
    },

    Mint(MintMsg<Extension>),
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
}

impl From<ExecuteMsg> for Cw721ExecuteMsg {
    fn from(msg: ExecuteMsg) -> Cw721ExecuteMsg {
        match msg {
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::Revoke { spender, token_id } => {
                Cw721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                Cw721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::RevokeAll { operator } => Cw721ExecuteMsg::RevokeAll { operator },
            _ => panic!("cannot convert {:?} to Cw721ExecuteMsg", msg),
        }
    }
}
