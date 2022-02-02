#![cfg(feature = "test-bpf")]

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

#[test]
fn test_validator_transaction() {
    solana_logger::setup_with_default("solana_program_runtime=debug");
    let program_id = Pubkey::new_unique();

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("bpf_program_template", program_id)
        .start();
    let rpc_client = test_validator.get_rpc_client();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(sysvar::slot_history::id(), false),
                AccountMeta::new(solana_program::incinerator::id(), false),
            ],
            data: vec![],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}
