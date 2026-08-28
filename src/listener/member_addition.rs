use std::env;

use serenity::all::ChannelId;
use serenity::all::Member;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let welcome_channel_id: u64 = env::var("WELCOME_CHANNEL_ID")
            .expect("Expected WELCOME_CHANNEL_ID in enviroment")
            .parse()
            .unwrap();
        let role_id: u64 = env::var("ROLE_ID")
            .expect("Expected ROLE_ID in enviroment")
            .parse()
            .unwrap();

        if let Err(e) = new_member.add_role(&ctx.http, role_id).await {
            println!("Error adding role to new member: {e}");
        }

        let member_count = if let Some(guild) = new_member.guild_id.to_guild_cached(&ctx.cache) {
            guild.member_count
        } else {
            0
        };

        let channel: ChannelId = welcome_channel_id.into();

        let message = if member_count == 0 {
            format!("Welcome {} to Ethene Hosting!", new_member.user.mention())
        } else {
            format!(
                "Welcome {} to Ethene Hosting! We are now: {}!",
                new_member.user.mention(),
                member_count
            )
        };

        let _ = channel.say(&ctx.http, &message).await;
    }
}
