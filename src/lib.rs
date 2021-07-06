use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    let account_info_iter = &mut accounts.iter();
    let rent_sysvar_account_info = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(rent_sysvar_account_info)?;
    msg!(
        "Rent exemption minimum lamport balance for an account that's 123 bytes in size: {}",
        rent.minimum_balance(123)
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        assert_matches::*,
        solana_program::{
            instruction::{AccountMeta, Instruction},
            sysvar,
        },
        solana_program_test::*,
        solana_sdk::{signature::Signer, transaction::Transaction},
    };

    #[tokio::test]
    async fn test_transaction() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new_readonly(sysvar::rent::id(), false)],
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
}
