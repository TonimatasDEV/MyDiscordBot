mod command;
mod listener;

use std::env;

use serenity::{
    Client,
    all::{ActivityData, GatewayIntents, Settings as CachedSettings},
};
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in enviroment");

    let mut client = Client::builder(&token, GatewayIntents::all())
        .cache_settings(CachedSettings::default())
        .activity(ActivityData::playing("Minecraft"))
        .event_handler(listener::member_addition::Handler)
        .event_handler(listener::ready_message::Handler)
        .event_handler(listener::commands::Handler)
        .await
        .expect("Err creating client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c");

        println!("Shutting down gracefully...");

        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Bot error: {why:?}")
    }
}
