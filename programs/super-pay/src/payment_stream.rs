use std::time::{SystemTime, UNIX_EPOCH};

use borsh::{BorshDeserialize, BorshSerialize};
use rocket::serde::ser::{Serialize, SerializeStruct};
use serde::Deserialize;
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey};
#[derive(Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PaymentStreams {
    pub end_time: UnixTimestamp,
    pub start_time: UnixTimestamp,
    pub amount_second: i64,
    pub to: Pubkey,
    pub from: Pubkey,
    pub lamports_withdrawn: i64,
    pub is_active: bool,
}

pub struct PaymentStreamResponse {
    pub end_time: UnixTimestamp,
    pub start_time: UnixTimestamp,
    pub amount_second: i64,
    pub to: Pubkey,
    pub from: Pubkey,
    pub lamports_withdrawn: i64,
    pub is_active: bool,
    pub id: Pubkey,
    pub yeild_earned: i64,
}

impl PaymentStreamResponse {
    pub fn new(stream: PaymentStreams, key: &Pubkey, balance: i64) -> Self {
        let yeild_earned = balance + stream.lamports_withdrawn
            - ((stream.end_time - stream.start_time) * stream.amount_second);

        PaymentStreamResponse {
            end_time: stream.end_time,
            start_time: stream.start_time,
            amount_second: stream.amount_second,
            from: stream.from,
            to: stream.to,
            id: *key,
            is_active: stream.is_active,
            lamports_withdrawn: stream.lamports_withdrawn,
            yeild_earned,
        }
    }
}

impl Serialize for PaymentStreamResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("PaymentStream", 7)?;
        s.serialize_field("end_time", &self.end_time)?;
        s.serialize_field("start_time", &self.start_time)?;
        s.serialize_field("amount_second", &self.amount_second)?;
        s.serialize_field(
            "total_amount",
            &((self.end_time - self.start_time) * self.amount_second),
        )?;
        s.serialize_field("lamports_withdrawn", &self.lamports_withdrawn)?;
        s.serialize_field("is_active", &self.is_active)?;
        let to_string = &self.to.to_string();
        let from_string = &self.from.to_string();
        let id_string = &self.id.to_string();
        s.serialize_field("to", to_string)?;
        s.serialize_field("from", from_string)?;
        s.serialize_field("id", id_string)?;
        s.serialize_field("yeildEarned", &self.yeild_earned)?;
        let mut status = "Completed";
        let mut status_id = 3;
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        if self.is_active
            && since_the_epoch.as_secs() > (self.start_time as u64)
            && since_the_epoch.as_secs() < self.end_time as u64
        {
            status = "Streaming";
            status_id = 1;
        }
        if (self.start_time as u64) > since_the_epoch.as_secs() {
            status = "Starting soon";
            status_id = 0;
        }

        s.serialize_field("status", status)?;
        s.serialize_field("statusID", &status_id)?;
        s.end()
    }
}
