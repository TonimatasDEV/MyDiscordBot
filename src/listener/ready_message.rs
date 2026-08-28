use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};

use crate::config::guild;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Started correctly: {}", ready.user.name);

        for ele in ready.guilds {
            guild::init_config_file(ele.id.get());
        }
    }
}
