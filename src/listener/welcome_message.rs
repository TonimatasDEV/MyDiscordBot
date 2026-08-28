use serenity::all::ChannelId;
use serenity::all::Member;
use serenity::async_trait;
use serenity::prelude::*;

use crate::config::guild::get_config;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let guild_id = new_member.guild_id.get();
        let config = get_config(guild_id);

        if !config.welcome_message_system || config.welcome_message_id == 0 {
            return;
        }

        let member_count = if let Some(guild) = new_member.guild_id.to_guild_cached(&ctx.cache) {
            guild.member_count
        } else {
            0
        };

        let channel: ChannelId = config.welcome_message_id.into();

        let guild_name = new_member.guild_id.name(&ctx.cache).unwrap();
        let message = if member_count == 0 {
            format!("Welcome {} to {}!", new_member.user.mention(), guild_name)
        } else {
            format!(
                "Welcome {} to {}! We are now: {}!",
                new_member.user.mention(),
                guild_name,
                member_count
            )
        };

        let _ = channel.say(&ctx.http, &message).await;
    }
}
