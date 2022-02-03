use crate::yield_models::PaymentStreams;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use rocket::{
    post,
    serde::json::{serde_json::json, Json, Value},
};
use serde::Deserialize;
#[post("/yield/stream", data = "<stream>")]
pub fn serialize_stream_yield(stream: Json<PaymentStreams>) -> Json<Value> {
    let temp = stream.0;
    let res = temp.try_to_vec().unwrap();
    Json(json!({"code": 200,"result":res,"length":res.len()}))
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Deserialize)]
pub struct UnstakeAmount {
    amount: i64,
}

#[post("/unstake", data = "<unstake>")]
pub fn unstake_serialize(unstake: Json<UnstakeAmount>) -> Json<Value> {
    let temp = unstake.0;
    let res = temp.try_to_vec().unwrap();
    Json(json!({"code": 200,"result":res}))
}
