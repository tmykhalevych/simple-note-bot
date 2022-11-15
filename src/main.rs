mod db;
mod schema;
mod models;
mod controllers;
mod views;
mod log;

use dotenv::dotenv;
use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    log::init();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let mut db_connection = db::establish_connection();
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        log::debug!("New update received: {:?}", &update);
        if let Some(mut controller) = controllers::Builder::new(update).with(&mut db_connection, &api) {
            controller.handle().await;
            if let Some(view) = controller.get_view() {
                log::debug!("Applying new view: {:?}", &view);
                view.render().await;
            }
        }
    }

    Ok(())
}
