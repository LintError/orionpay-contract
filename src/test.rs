#![cfg(test)]
use super::*;
use crate::payments::Payments;
use crate::shared::{OrionError, Status};
use crate::split_payments::{SplitRecipient, create_split_payment, get_split_payment};
use soroban_sdk::{Address, Env, String, testutils::Address as _};

#[test]
fn test_version_and_hello() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    let client = OrionPayClient::new(&env, &contract_id);

    // Test version
    let version = client.version();
    assert_eq!(version, String::from_str(&env, env!("CARGO_PKG_VERSION")));

    // Test hello
    let result = client.hello(&String::from_str(&env, "Orion"));
    assert_eq!(result, String::from_str(&env, "Hello Orion"));
}

#[test]
fn test_payment_create_complete_get() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Create payment (in contract context)
    let payment = env
        .as_contract(&contract_id, || {
            Payments::create(
                env.clone(),
                sender.clone(),
                recipient.clone(),
                1000,
                String::from_str(&env, "USDC"),
                Some(String::from_str(&env, "test payment")),
            )
        })
        .unwrap();

    assert_eq!(payment.sender, sender);
    assert_eq!(payment.recipient, recipient);
    assert_eq!(payment.amount, 1000);
    assert!(payment.completed_at.is_none());

    // Complete payment
    env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment.id.clone())
    })
    .unwrap();

    // Get payment and verify completed
    let retrieved = env
        .as_contract(&contract_id, || {
            Payments::get_payment(env.clone(), payment.id.clone())
        })
        .unwrap();
    assert!(retrieved.completed_at.is_some());
}

#[test]
fn test_payment_invalid_amount() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Attempt to create payment with zero amount
    let result = env.as_contract(&contract_id, || {
        Payments::create(
            env.clone(),
            sender.clone(),
            recipient.clone(),
            0,
            String::from_str(&env, "USDC"),
            None,
        )
    });

    assert_eq!(result.unwrap_err(), OrionError::InvalidAmount);
}

#[test]
fn test_payment_not_found() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());

    let result = env.as_contract(&contract_id, || {
        Payments::get_payment(env.clone(), String::from_str(&env, "nonexistent"))
    });
    assert_eq!(result.unwrap_err(), OrionError::NotFound);
}

#[test]
fn test_split_payment_percentage_validation() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());

    let mut recipients = soroban_sdk::Vec::new(&env);
    recipients.push_back(SplitRecipient {
        address: String::from_str(&env, "addr1"),
        percentage: 50,
        amount: 0,
        completed: false,
    });
    recipients.push_back(SplitRecipient {
        address: String::from_str(&env, "addr2"),
        percentage: 30, // Only 80% total - should fail
        amount: 0,
        completed: false,
    });

    let result = env.as_contract(&contract_id, || {
        create_split_payment(
            env.clone(),
            String::from_str(&env, "sender"),
            1000,
            recipients,
        )
    });

    assert_eq!(result.unwrap_err(), OrionError::InvalidPercentage);
}

#[test]
fn test_split_payment_create_and_get() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());

    let mut recipients = soroban_sdk::Vec::new(&env);
    recipients.push_back(SplitRecipient {
        address: String::from_str(&env, "addr1"),
        percentage: 60,
        amount: 0,
        completed: false,
    });
    recipients.push_back(SplitRecipient {
        address: String::from_str(&env, "addr2"),
        percentage: 40,
        amount: 0,
        completed: false,
    });

    let split_id = env
        .as_contract(&contract_id, || {
            create_split_payment(
                env.clone(),
                String::from_str(&env, "sender"),
                1000,
                recipients.clone(),
            )
        })
        .unwrap();

    let split = env
        .as_contract(&contract_id, || {
            get_split_payment(env.clone(), split_id.clone())
        })
        .unwrap();

    assert_eq!(split.total_amount, 1000);
    assert_eq!(split.status, Status::Pending);
    assert_eq!(split.recipients.len(), 2);
    // Verify amounts were calculated correctly
    assert_eq!(split.recipients.get(0).unwrap().amount, 600);
    assert_eq!(split.recipients.get(1).unwrap().amount, 400);
}
