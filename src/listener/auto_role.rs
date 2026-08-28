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

        if !config.auto_role_system || config.auto_role_id == 0 {
            return;
        }

        if let Err(e) = new_member.add_role(&ctx.http, config.auto_role_id).await {
            println!("Error adding role to new member: {e}");
        }
    }
}
