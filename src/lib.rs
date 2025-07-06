use solana_program::{
    account_info:: {next_account_info, AccountInfo},
    entrypoint::ProgramResult,entrypoint , msg, pubkey::Pubkey
};

use borsh::{BorshDeserialize, BorshSerialize};

entrypoint!(counter_program);


#[derive(BorshDeserialize, BorshSerialize, Debug)]
enum InstructionType {
    Increment (u32),
    Decrement (u32)
}


#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Counter {
    count :u32
}


pub fn counter_program(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data :&[u8]
)-> ProgramResult {
    

    let account_info = next_account_info(&mut accounts.iter())?;


    let instruction = InstructionType::try_from_slice(instruction_data)?;


    let mut counter_data = Counter::try_from_slice(&mut account_info.data.borrow())?;

    match instruction {
        InstructionType::Increment(value) => {
            counter_data.count+=value;

        },
        InstructionType::Decrement(value)=>{
            counter_data.count-=value;
        }
    }

    counter_data.serialize(&mut *account_info.data.borrow_mut());
    msg!("Counter updated to {}", counter_data.count);

    Ok(())
}   