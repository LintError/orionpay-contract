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
    assert_eq!(version, String::from_str(&env, "0.1.0"));

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

    // Complete payment, authorized by the recipient
    env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment.id.clone(), recipient.clone())
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
        address: Address::generate(&env),
        percentage: 50,
        amount: 0,
        completed: false,
    });
    recipients.push_back(SplitRecipient {
        address: Address::generate(&env),
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
        address: Address::generate(&env),
        percentage: 60,
        amount: 0,
        completed: false,
    });
    recipients.push_back(SplitRecipient {
        address: Address::generate(&env),
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

// ---------------------------------------------------------------------------
// Record ID uniqueness (#31, #32)
// ---------------------------------------------------------------------------

#[test]
fn test_payment_ids_are_unique_within_one_ledger() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Both payments are created in the same ledger (same timestamp).
    let first = env
        .as_contract(&contract_id, || {
            Payments::create(
                env.clone(),
                sender.clone(),
                recipient.clone(),
                10,
                String::from_str(&env, "XLM"),
                None,
            )
        })
        .unwrap();

    let second = env
        .as_contract(&contract_id, || {
            Payments::create(
                env.clone(),
                sender.clone(),
                recipient.clone(),
                99,
                String::from_str(&env, "XLM"),
                None,
            )
        })
        .unwrap();

    // IDs must differ, otherwise the second create overwrites the first record.
    assert_ne!(first.id, second.id);

    // Both records must still be retrievable with their original amounts.
    let first_stored = env
        .as_contract(&contract_id, || {
            Payments::get_payment(env.clone(), first.id.clone())
        })
        .unwrap();
    let second_stored = env
        .as_contract(&contract_id, || {
            Payments::get_payment(env.clone(), second.id.clone())
        })
        .unwrap();

    assert_eq!(first_stored.amount, 10);
    assert_eq!(second_stored.amount, 99);
}

#[test]
fn test_split_and_bulk_ids_are_namespaced() {
    use crate::bulk_payments::{BulkPaymentRecipient, create_bulk_payment};

    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let mut recipients = soroban_sdk::Vec::new(&env);
    recipients.push_back(SplitRecipient {
        address: Address::generate(&env),
        percentage: 100,
        amount: 0,
        completed: false,
    });

    let mut bulk_recipients = soroban_sdk::Vec::new(&env);
    bulk_recipients.push_back(BulkPaymentRecipient {
        address: String::from_str(&env, "addr"),
        amount: 5,
        completed: false,
    });

    // Both are created in the same ledger, so timestamps are identical.
    let split_id = env
        .as_contract(&contract_id, || {
            create_split_payment(
                env.clone(),
                String::from_str(&env, "sender"),
                100,
                recipients,
            )
        })
        .unwrap();

    let bulk_id = env
        .as_contract(&contract_id, || {
            create_bulk_payment(
                env.clone(),
                String::from_str(&env, "sender"),
                bulk_recipients,
            )
        })
        .unwrap();

    // Namespaces must keep the two record types apart.
    assert_ne!(split_id, bulk_id);

    // A split ID must not be readable through the bulk accessor, and vice versa.
    let as_bulk = env.as_contract(&contract_id, || {
        crate::bulk_payments::get_bulk_payment(env.clone(), split_id.clone())
    });
    assert_eq!(as_bulk.unwrap_err(), OrionError::NotFound);

    let as_split = env.as_contract(&contract_id, || {
        get_split_payment(env.clone(), bulk_id.clone())
    });
    assert_eq!(as_split.unwrap_err(), OrionError::NotFound);
}

#[test]
fn test_two_splits_in_one_ledger_get_distinct_ids() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let mut recipients_a = soroban_sdk::Vec::new(&env);
    recipients_a.push_back(SplitRecipient {
        address: Address::generate(&env),
        percentage: 100,
        amount: 0,
        completed: false,
    });

    let mut recipients_b = soroban_sdk::Vec::new(&env);
    recipients_b.push_back(SplitRecipient {
        address: Address::generate(&env),
        percentage: 100,
        amount: 0,
        completed: false,
    });

    let id_a = env
        .as_contract(&contract_id, || {
            create_split_payment(env.clone(), String::from_str(&env, "s1"), 100, recipients_a)
        })
        .unwrap();

    let id_b = env
        .as_contract(&contract_id, || {
            create_split_payment(env.clone(), String::from_str(&env, "s2"), 777, recipients_b)
        })
        .unwrap();

    assert_ne!(id_a, id_b);

    // The first split must not have been clobbered by the second.
    let split_a = env
        .as_contract(&contract_id, || {
            get_split_payment(env.clone(), id_a.clone())
        })
        .unwrap();
    assert_eq!(split_a.total_amount, 100);
}

// ---------------------------------------------------------------------------
// Authorization (#29)
// ---------------------------------------------------------------------------

/// Create a payment and return its id along with the sender/recipient pair.
fn create_payment_fixture(
    env: &Env,
    contract_id: &soroban_sdk::Address,
) -> (String, Address, Address) {
    let sender = Address::generate(env);
    let recipient = Address::generate(env);
    let payment = env
        .as_contract(contract_id, || {
            Payments::create(
                env.clone(),
                sender.clone(),
                recipient.clone(),
                1000,
                String::from_str(env, "XLM"),
                None,
            )
        })
        .unwrap();
    (payment.id, sender, recipient)
}

#[test]
fn test_complete_rejects_unauthorized_caller() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (payment_id, _sender, _recipient) = create_payment_fixture(&env, &contract_id);
    let stranger = Address::generate(&env);

    let result = env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment_id.clone(), stranger)
    });

    assert_eq!(result.unwrap_err(), OrionError::Unauthorized);

    // The payment must remain incomplete after a rejected completion.
    let payment = env
        .as_contract(&contract_id, || {
            Payments::get_payment(env.clone(), payment_id)
        })
        .unwrap();
    assert!(payment.completed_at.is_none());
}

#[test]
fn test_complete_succeeds_for_sender_and_recipient() {
    // Either party may complete: the recipient claims, or the sender confirms.
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    // Completed by the recipient.
    let (payment_id, _sender, recipient) = create_payment_fixture(&env, &contract_id);
    env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment_id.clone(), recipient.clone())
    })
    .unwrap();
    let payment = env
        .as_contract(&contract_id, || {
            Payments::get_payment(env.clone(), payment_id)
        })
        .unwrap();
    assert!(payment.completed_at.is_some());

    // Completed by the sender.
    let (payment_id, sender, _recipient) = create_payment_fixture(&env, &contract_id);
    env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment_id.clone(), sender.clone())
    })
    .unwrap();
    let payment = env
        .as_contract(&contract_id, || {
            Payments::get_payment(env.clone(), payment_id)
        })
        .unwrap();
    assert!(payment.completed_at.is_some());
}

#[test]
fn test_complete_twice_returns_invalid_state() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (payment_id, _sender, recipient) = create_payment_fixture(&env, &contract_id);

    env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment_id.clone(), recipient.clone())
    })
    .unwrap();

    let second = env.as_contract(&contract_id, || {
        Payments::complete(env.clone(), payment_id, recipient)
    });
    assert_eq!(second.unwrap_err(), OrionError::InvalidState);
}

// ---------------------------------------------------------------------------
// Authorization (#33)
// ---------------------------------------------------------------------------

/// Create a two-recipient 50/50 split and return its id plus both recipients.
fn create_split_fixture(
    env: &Env,
    contract_id: &soroban_sdk::Address,
) -> (String, Address, Address) {
    let alice = Address::generate(env);
    let bob = Address::generate(env);

    let mut recipients = soroban_sdk::Vec::new(env);
    recipients.push_back(SplitRecipient {
        address: alice.clone(),
        percentage: 50,
        amount: 0,
        completed: false,
    });
    recipients.push_back(SplitRecipient {
        address: bob.clone(),
        percentage: 50,
        amount: 0,
        completed: false,
    });

    let id = env
        .as_contract(contract_id, || {
            create_split_payment(
                env.clone(),
                String::from_str(env, "sender"),
                1000,
                recipients,
            )
        })
        .unwrap();

    (id, alice, bob)
}

#[test]
fn test_split_process_recipient_succeeds_for_the_recipient() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (split_id, alice, _bob) = create_split_fixture(&env, &contract_id);

    env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 0, alice.clone())
    })
    .unwrap();

    let split = env
        .as_contract(&contract_id, || get_split_payment(env.clone(), split_id))
        .unwrap();

    assert_eq!(split.completed_count, 1);
    assert!(split.recipients.get(0).unwrap().completed);
    assert!(!split.recipients.get(1).unwrap().completed);
    assert_eq!(split.status, Status::Processing);
}

#[test]
fn test_split_process_recipient_rejects_third_party() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (split_id, _alice, _bob) = create_split_fixture(&env, &contract_id);
    let stranger = Address::generate(&env);

    let result = env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 0, stranger)
    });
    assert_eq!(result.unwrap_err(), OrionError::Unauthorized);

    // `completed_count` must not advance on an unauthorized call.
    let split = env
        .as_contract(&contract_id, || get_split_payment(env.clone(), split_id))
        .unwrap();
    assert_eq!(split.completed_count, 0);
    assert_eq!(split.status, Status::Pending);
}

#[test]
fn test_split_process_recipient_rejects_other_recipient_of_same_split() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (split_id, alice, bob) = create_split_fixture(&env, &contract_id);

    // Bob is a party to this split, but he is not the recipient at index 0.
    let result = env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 0, bob)
    });
    assert_eq!(result.unwrap_err(), OrionError::Unauthorized);

    // Alice may still process her own index.
    env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id, 0, alice)
    })
    .unwrap();
}

#[test]
fn test_split_process_recipient_rejects_out_of_range_index() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (split_id, _alice, _bob) = create_split_fixture(&env, &contract_id);

    // Authorized signer, but index 5 is past the end of the recipients vector.
    let signer = Address::generate(&env);
    let result = env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id, 5, signer)
    });
    assert_eq!(result.unwrap_err(), OrionError::InvalidRecipient);
}

#[test]
fn test_split_process_recipient_does_not_double_count() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();

    let (split_id, alice, bob) = create_split_fixture(&env, &contract_id);

    // Process index 0 twice.
    env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 0, alice.clone())
    })
    .unwrap();
    env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 0, alice)
    })
    .unwrap();

    let split = env
        .as_contract(&contract_id, || {
            get_split_payment(env.clone(), split_id.clone())
        })
        .unwrap();
    assert_eq!(split.completed_count, 1);
    assert_eq!(split.status, Status::Processing);

    // Completing the second recipient finishes the split exactly once.
    env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 1, bob.clone())
    })
    .unwrap();
    env.as_contract(&contract_id, || {
        crate::split_payments::process_recipient(env.clone(), split_id.clone(), 1, bob)
    })
    .unwrap();

    let split = env
        .as_contract(&contract_id, || get_split_payment(env.clone(), split_id))
        .unwrap();
    assert_eq!(split.completed_count, 2);
    assert_eq!(split.status, Status::Completed);
}
