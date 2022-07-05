use {
    assert_matches::*,
    anchor_lang::{ToAccountMetas,InstructionData},
    solana_program::instruction::{AccountMeta, Instruction},
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_program::{pubkey::Pubkey}
};

#[tokio::test]
async fn test_success() {
                      assert!(true);
                                    }

#[tokio::test]
async fn test_instruction_call() {
    let program_id = anchor_test_poc::ID;

    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "anchor_test_poc",
        program_id,
        processor!(anchor_test_poc::entry)
    )
        .start()
        .await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: anchor_test_poc::accounts::Initialize {}.to_account_metas(None),
            data: anchor_test_poc::instruction::Initialize{}.data()
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
}