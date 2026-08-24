mod event;

use serenity::{
    Client,
    all::{ActivityData, GatewayIntents, Settings as CachedSettings},
};
use tokio::signal;

#[tokio::main]
async fn main() {
    let token = "MTM4Njc5NTAxOTQxMDYwODI1OQ.G8D7Dr.Wc4aHcRxuVD1lP72SU_k47KkAehhJoOvQjC48A";

    let mut client = Client::builder(&token, GatewayIntents::all())
        .cache_settings(CachedSettings::default())
        .activity(ActivityData::playing("Minecraft"))
        .event_handler(event::Handler)
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
