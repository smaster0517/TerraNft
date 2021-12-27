#[cfg(test)]
mod static_stub {
    use cosmwasm_std::testing::{mock_env, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::Addr;

    use terra_multi_test::{App, BankKeeper, ContractWrapper, Executor, TerraMockQuerier};

    use cw721::{AllNftInfoResponse, Cw721QueryMsg, NftInfoResponse};
    use cw721_metadata_onchain::Metadata;

    use crate::entry::InstantiateMsg;

    /// Lifted from astroport
    /// https://github.com/astroport-fi/astroport-lbport/blob/ee24a0c532ec01a8af61ef58d5efc689bded1a16/contracts/factory/tests/integration.rs#L13
    fn mock_app() -> App {
        let env = mock_env();
        let api = MockApi::default();
        let bank = BankKeeper::new();
        let storage = MockStorage::new();
        let terra_mock_querier = TerraMockQuerier::new(MockQuerier::new(&[]));

        App::new(api, env.block, bank, storage, terra_mock_querier)
    }

    fn init_contract(app: &mut App) -> u64 {
        let contract = Box::new(ContractWrapper::new(
            crate::entry::execute,
            crate::entry::instantiate,
            crate::entry::query,
        ));

        app.store_code(contract)
    }

    fn create_contract(app: &mut App, code_id: u64) -> Addr {
        // This is the test1 address from localterra. There is probably a better way to create a
        // mock / stub address for testing but needs to be investigated.
        let owner = "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";

        let msg = InstantiateMsg {
            always_owner: owner.to_string(),
            minter: owner.to_string(),
            name: "test1".to_string(),
            symbol: "TEST1".to_string(),
            static_token: None,
        };

        app.instantiate_contract(
            code_id,
            Addr::unchecked("admin1"),
            &msg,
            &[],
            "TerraStubNft",
            None,
        )
        .unwrap()
    }

    #[test]
    fn nft_info_from_query_wasm_smart() {
        let mut app = mock_app();
        let code_id = init_contract(&mut app);

        // instantiate
        let nft_info = Cw721QueryMsg::NftInfo {
            token_id: "stub".to_string(),
        };

        let contract = create_contract(&mut app, code_id);

        // test that query_wasm_smart will succesfully return an NftInfoResponse
        let info: NftInfoResponse<Metadata> =
            app.wrap().query_wasm_smart(contract, &nft_info).unwrap();

        assert_eq!(
            Some("https://stub.test/stub_token.json".to_string()),
            info.token_uri
        );
    }

    #[test]
    fn all_nft_info_from_query_wasm_smart() {
        let mut app = mock_app();
        let code_id = init_contract(&mut app);

        // instantiate
        let all_nft_info = Cw721QueryMsg::AllNftInfo {
            token_id: "stub".to_string(),
            include_expired: None,
        };

        let contract = create_contract(&mut app, code_id);

        // test that query_wasm_smart will succesfully return an NftInfoResponse
        let resp: AllNftInfoResponse<Metadata> = app
            .wrap()
            .query_wasm_smart(contract, &all_nft_info)
            .unwrap();

        assert_eq!(
            Some("https://stub.test/stub_token.json".to_string()),
            resp.info.token_uri
        );
    }
}
