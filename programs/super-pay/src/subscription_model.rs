use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Deserialize, Serialize)]
pub struct ReciverRewardPercentage {
    pub percentage: u8,
}
