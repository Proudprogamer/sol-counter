use solana_program::{
    account_info:: {next_account_info, AccountInfo},
    entrypoint::ProgramResult,entrypoint , msg, pubkey::Pubkey
};

entrypoint!(counter_program);

pub fn counter_program(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data :&[u8]
)-> ProgramResult {
    Ok(())
    
}