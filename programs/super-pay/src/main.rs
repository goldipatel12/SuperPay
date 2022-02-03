mod payment_stream;
mod reciver_model;
mod routes;
mod solana;
mod withdraw_model;
use crate::routes::reciver_reward_serialize;
use crate::routes::serialize_stream;
use crate::routes::withdraw_serialize;
use crate::yield_routes::serialize_stream_yield;
use crate::yield_routes::unstake_serialize;
use routes::get_streams;
use routes::index;
mod stream_u8;
mod yield_models;
mod yield_routes;
use rocket::routes;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_streams,
                serialize_stream,
                reciver_reward_serialize,
                withdraw_serialize,
                serialize_stream_yield,
                unstake_serialize
            ],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
