#[cfg(test)]
mod test {
    use svm_codec::Codec;
    use svm_genesis_config::GenesisConfig;
    use svm_runtime::{PriceResolverRegistry, Runtime, TemplatePriceCache};
    use svm_state::GlobalState;
    use svm_types::{
        Address, BytesPrimitive, CodeKind, CodeSection, Context, CtorsSection, DataSection,
        Envelope, Gas, GasMode, Layer, Section, Sections, State, TemplateAddr, Transaction,
        TransactionId,
    };

    fn deploy_sct_template_helper(runtime: &mut Runtime) -> TemplateAddr {
        let principal = Address::zeros();
        let gas_limit = Gas::new();
        let gas_fee = 0;
        let envelope = Envelope::new(principal, 0, 0, gas_limit, gas_fee);

        let message = svm_genesis_config::sct().1.sections().encode_to_vec();

        let tx_id = TransactionId::zeros();
        let layer = Layer::default();
        let state = State::zeros();
        let context = Context::new(tx_id, layer, state);

        let receipt = runtime.deploy(&envelope, &message, &context);
        assert!(receipt.success);
        receipt.addr.unwrap()
    }

    #[test]
    fn deploy_sct_template() {
        let gs = GlobalState::in_memory(GenesisConfig::mainnet());
        let pricing_registry = PriceResolverRegistry::default();
        let pricing_cache = TemplatePriceCache::new(pricing_registry);
        let mut runtime = Runtime::new(gs, pricing_cache);
        let sct_template_addr = deploy_sct_template_helper(&mut runtime);

        let addr_1 = Address::repeat(0x4c);
        let addr_2 = Address::repeat(0x4d);

        runtime
            .create_genesis_account(&addr_1, "addr_1", 100, 0)
            .unwrap();
        runtime
            .create_genesis_account(&addr_2, "addr_2", 100, 0)
            .unwrap();

        let tx_id = TransactionId::zeros();
        let envelope = Envelope::new(addr_1, 5, 0, Gas::new(), 0);
        let tx = Transaction {
            version: 0,
            target: addr_1,
            func_name: "transfer".to_string(),
            verifydata: vec![],
            calldata: vec![],
        };
        let context = Context::new(tx_id, Layer::default(), State::zeros());
        runtime.call(&envelope, &tx.encode_to_vec(), &context);
    }
}
