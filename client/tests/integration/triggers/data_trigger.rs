#![allow(clippy::restriction)]

use eyre::Result;
use iroha_client::client;
use iroha_data_model::prelude::*;
use test_network::*;

#[test]
fn must_execute_both_triggers() -> Result<()> {
    let (_rt, _peer, mut test_client) = <PeerBuilder>::new().start_with_runtime();
    wait_for_genesis_committed(&vec![test_client.clone()], 0);

    let account_id: AccountId = "alice@wonderland".parse()?;
    let asset_definition_id = "rose#wonderland".parse()?;
    let asset_id = AssetId::new(asset_definition_id, account_id.clone());

    let prev_value = get_asset_value(&mut test_client, asset_id.clone())?;

    let instruction = MintBox::new(1_u32, asset_id.clone());
    let register_trigger = RegisterBox::new(Trigger::new(
        "mint_rose_1".parse()?,
        Action::new(
            Executable::from(vec![instruction.clone().into()]),
            Repeats::Indefinitely,
            account_id.clone(),
            FilterBox::Data(BySome(DataEntityFilter::ByAccount(BySome(
                AccountFilter::new(AcceptAll, BySome(AccountEventFilter::ByCreated)),
            )))),
        ),
    ));
    test_client.submit_blocking(register_trigger)?;

    let register_trigger = RegisterBox::new(Trigger::new(
        "mint_rose_2".parse()?,
        Action::new(
            Executable::from(vec![instruction.into()]),
            Repeats::Indefinitely,
            account_id,
            FilterBox::Data(BySome(DataEntityFilter::ByDomain(BySome(
                DomainFilter::new(AcceptAll, BySome(DomainEventFilter::ByCreated)),
            )))),
        ),
    ));
    test_client.submit_blocking(register_trigger)?;

    test_client.submit_blocking(RegisterBox::new(Account::new(
        "bunny@wonderland".parse()?,
        [],
    )))?;
    test_client.submit_blocking(RegisterBox::new(Domain::new("neverland".parse()?)))?;

    let new_value = get_asset_value(&mut test_client, asset_id)?;
    assert_eq!(new_value, prev_value + 2);

    Ok(())
}

fn get_asset_value(client: &mut client::Client, asset_id: AssetId) -> Result<u32> {
    let asset = client.request(client::asset::by_id(asset_id))?;
    Ok(*TryAsRef::<u32>::try_as_ref(asset.value())?)
}
