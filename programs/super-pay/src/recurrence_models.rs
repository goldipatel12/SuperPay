use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Deserialize)]
pub struct PaymentStreams {
    pub end_time: UnixTimestamp,
    pub start_time: UnixTimestamp,
    pub amount_second: i64,
    pub to: Pubkey,
    pub from: Pubkey,
    pub lamports_withdrawn: i64,
    pub is_active: bool,
    pub stake_account: Pubkey,
    pub is_delegated: bool,
    pub delegate_time: UnixTimestamp,
    pub vote_right_to: Pubkey,
    pub seed: String,
}
