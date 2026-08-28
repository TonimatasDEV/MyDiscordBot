use serenity::{
    all::{Command, Context, EventHandler, Interaction, Ready},
    async_trait,
};

use crate::command;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ping" => command::ping::run(ctx, command).await,
                "set-auto-role" => command::set_auto_role::run(ctx, command).await,
                "set-welcome-channel" => command::set_welcome_channel::run(ctx, command).await,
                "system-config" => command::system_config::run(ctx, command).await,
                _ => println!("invalid command"),
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let _ = Command::create_global_command(&ctx.http, command::ping::register()).await;
        let _ = Command::create_global_command(&ctx.http, command::set_auto_role::register()).await;
        let _ = Command::create_global_command(&ctx.http, command::set_welcome_channel::register())
            .await;
        let _ = Command::create_global_command(&ctx.http, command::system_config::register()).await;
    }
}
