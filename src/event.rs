use serenity::all::ChannelId;
use serenity::all::Member;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::prelude::*;

const WELCOME_CHANNEL_ID: u64 = 1323369443145027738;
const ROLE_ID: u64 = 1323370508477005865;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Err(e) = new_member.add_role(&ctx.http, ROLE_ID).await {
            println!("Error adding role to new member: {e}");
        }

        let member_count = if let Some(guild) = new_member.guild_id.to_guild_cached(&ctx.cache) {
            guild.member_count
        } else {
            0
        };

        let channel: ChannelId = WELCOME_CHANNEL_ID.into();

        let message = format!(
            "Welcome {} to Ethene Hosting! We are now: {}!",
            new_member.user.mention(),
            member_count
        );

        let _ = channel.say(&ctx.http, &message).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}
