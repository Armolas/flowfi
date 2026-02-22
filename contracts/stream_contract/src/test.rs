#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, token, xdr, Address, Env, Symbol, TryFromVal};

fn create_token_contract(env: &Env) -> (Address, Address) {
    let admin = Address::generate(env);
    let token = env.register_stellar_asset_contract_v2(admin.clone());
    (token.address(), admin)
}

#[test]
fn test_create_stream_persists_state() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &1_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);

    let token_client = token::Client::new(&env, &token_address);
    token_client.approve(&sender, &contract_id, &500, &1_000_000);

    let amount: i128 = 500;
    let duration: u64 = 100;
    let stream_id = client.create_stream(&sender, &recipient, &token_address, &amount, &duration);

    assert_eq!(stream_id, 1);

    let stream = client.get_stream(&stream_id).unwrap();
    assert_eq!(stream.sender, sender);
    assert_eq!(stream.recipient, recipient);
    assert_eq!(stream.token_address, token_address);
    assert_eq!(stream.rate_per_second, amount / duration as i128);
    assert_eq!(stream.deposited_amount, amount);
    assert_eq!(stream.withdrawn_amount, 0);
    assert!(stream.is_active);
}

#[test]
fn test_create_multiple_streams_increments_counter() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &2_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);

    let token_client = token::Client::new(&env, &token_address);
    token_client.approve(&sender, &contract_id, &2_000, &1_000_000);

    let stream_id1 = client.create_stream(&sender, &recipient1, &token_address, &500, &100);
    let stream_id2 = client.create_stream(&sender, &recipient2, &token_address, &500, &100);

    assert_eq!(stream_id1, 1);
    assert_eq!(stream_id2, 2);
    assert!(client.get_stream(&stream_id1).is_some());
    assert!(client.get_stream(&stream_id2).is_some());
}

#[test]
fn test_create_stream_transfers_tokens() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &1_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);
    let token_client = token::Client::new(&env, &token_address);

    let initial_sender_balance = token_client.balance(&sender);
    let initial_contract_balance = token_client.balance(&contract_id);

    token_client.approve(&sender, &contract_id, &500, &1_000_000);

    let amount: i128 = 500;
    client.create_stream(&sender, &recipient, &token_address, &amount, &100);

    assert_eq!(
        token_client.balance(&sender),
        initial_sender_balance - amount
    );
    assert_eq!(
        token_client.balance(&contract_id),
        initial_contract_balance + amount
    );
}

#[test]
fn test_top_up_stream_success() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &20_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);
    let token_client = token::Client::new(&env, &token_address);
    token_client.approve(&sender, &contract_id, &20_000, &1_000_000);

    let stream_id = client.create_stream(&sender, &recipient, &token_address, &10_000, &100);

    let top_up_amount = 5_000;
    let result = client.try_top_up_stream(&sender, &stream_id, &top_up_amount);
    assert!(result.is_ok());

    let stream = client.get_stream(&stream_id).unwrap();
    assert_eq!(stream.deposited_amount, 15_000);
}

#[test]
fn test_top_up_stream_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &20_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);
    let token_client = token::Client::new(&env, &token_address);
    token_client.approve(&sender, &contract_id, &20_000, &1_000_000);

    let stream_id = client.create_stream(&sender, &recipient, &token_address, &10_000, &100);

    let negative_result = client.try_top_up_stream(&sender, &stream_id, &-100);
    assert_eq!(negative_result, Err(Ok(StreamError::InvalidAmount)));

    let zero_result = client.try_top_up_stream(&sender, &stream_id, &0);
    assert_eq!(zero_result, Err(Ok(StreamError::InvalidAmount)));
}

#[test]
fn test_top_up_stream_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let stream_id = 999_u64;

    let result = client.try_top_up_stream(&sender, &stream_id, &1_000);
    assert_eq!(result, Err(Ok(StreamError::StreamNotFound)));
}

#[test]
fn test_top_up_stream_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let attacker = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &20_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);
    let token_client = token::Client::new(&env, &token_address);
    token_client.approve(&sender, &contract_id, &20_000, &1_000_000);

    let stream_id = client.create_stream(&sender, &recipient, &token_address, &10_000, &100);

    let result = client.try_top_up_stream(&attacker, &stream_id, &1_000);
    assert_eq!(result, Err(Ok(StreamError::Unauthorized)));
}

#[test]
fn test_top_up_stream_inactive() {
    let env = Env::default();
    env.mock_all_auths();

    let (token_address, _admin) = create_token_contract(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let stellar_asset = token::StellarAssetClient::new(&env, &token_address);
    stellar_asset.mint(&sender, &20_000);

    let contract_id = env.register(StreamContract, ());
    let client = StreamContractClient::new(&env, &contract_id);
    let token_client = token::Client::new(&env, &token_address);
    token_client.approve(&sender, &contract_id, &20_000, &1_000_000);

    let stream_id = client.create_stream(&sender, &recipient, &token_address, &10_000, &100);
    client.cancel_stream(&sender, &stream_id);

    let result = client.try_top_up_stream(&sender, &stream_id, &1_000);
    assert_eq!(result, Err(Ok(StreamError::StreamInactive)));
}

#[test]
fn datakey_stream_serializes_deterministically_and_works_in_storage() {
    let env = Env::default();
    let contract_id = env.register(StreamContract, ());
    let key = DataKey::Stream(42_u64);

    let key_scval_a: xdr::ScVal = (&key).try_into().unwrap();
    let key_scval_b: xdr::ScVal = (&key).try_into().unwrap();
    assert_eq!(key_scval_a, key_scval_b);

    let expected_key_scval: xdr::ScVal =
        (&(Symbol::new(&env, "Stream"), 42_u64)).try_into().unwrap();
    assert_eq!(key_scval_a, expected_key_scval);

    let decoded_key = DataKey::try_from_val(&env, &key_scval_a).unwrap();
    assert_eq!(decoded_key, key);

    let stream = Stream {
        sender: Address::generate(&env),
        recipient: Address::generate(&env),
        token_address: Address::generate(&env),
        rate_per_second: 100,
        deposited_amount: 1_000,
        withdrawn_amount: 0,
        start_time: 1,
        last_update_time: 1,
        is_active: true,
    };

    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&key, &stream);
        let stored: Stream = env.storage().persistent().get(&key).unwrap();
        assert_eq!(stored, stream);
    });
}
