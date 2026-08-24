mod event;
use serenity::{Client, all::GatewayIntents};

#[tokio::main]
async fn main() {
    let token = "MTM4Njc5NTAxOTQxMDYwODI1OQ.G8D7Dr.Wc4aHcRxuVD1lP72SU_k47KkAehhJoOvQjC48A";
    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(event::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Bot error: {why:?}")
    }

    println!("Bot running correctly!");
}
