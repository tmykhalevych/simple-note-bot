mod db;
mod schema;
mod models;
mod controllers;

use dotenv::dotenv;
use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let mut db_connection = db::establish_connection();
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        match controllers::Builder::new(update).with(&mut db_connection, &api) {
            Ok(mut controller) => controller.handle().await,
            Err(err_msg) => println!("{err_msg}"),
        }
    }

    Ok(())
}
