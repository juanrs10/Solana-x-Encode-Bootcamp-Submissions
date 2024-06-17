use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Defines the structure of the state stored in the on-chain account
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, BorshSchema)]
pub struct GreetingStruct {
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the counter program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored
) -> ProgramResult {
    msg!("[lib] Solana Example2 counter program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let hello_account = next_account_info(accounts_iter)?;

    // Retrieve the lamport balance of the account
    let lamport_balance = hello_account.lamports();

    msg!("[lib] hello account: {:?}", hello_account.key);
    msg!("[lib] hello account balance: {} lamports", lamport_balance);

    // The account must be owned by the program in order to modify its data
    if hello_account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    } else {
        msg!("Greeted account has the correct program id");
    }

    // Create a struct that's easy to interact with programmatically from account data
    let mut greeting_struct = GreetingStruct::try_from_slice(&hello_account.data.borrow())?;

    // Increment by one
    greeting_struct.counter += 1;

    msg!(
        "Program added to the greeting counter struct stored at: {:?}",
        hello_account.key
    );

    // Serialize the local struct and store it back into the account
    greeting_struct.serialize(&mut &mut hello_account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_struct.counter);

    Ok(())
}
