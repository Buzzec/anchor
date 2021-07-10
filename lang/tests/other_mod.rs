use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(cool: u8)]
pub struct TestCall<'info>{
    pub account: AccountInfo<'info>,
    #[account(mut, seeds = [cool])]
    pub data_account: ProgramAccount<'info, OtherModAccount>,
}

#[account]
pub struct OtherModAccount{
    pub data1: u8,
    pub data2: u64,
}
