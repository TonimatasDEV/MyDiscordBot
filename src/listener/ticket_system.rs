use std::time::Duration;

use serenity::{
    all::{
        ButtonStyle, Channel, ChannelId, Colour, Context, CreateActionRow, CreateButton,
        CreateChannel, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage, EditChannel, EventHandler, GetMessages,
        Interaction, Mentionable, PermissionOverwrite, PermissionOverwriteType, Permissions,
        ReactionType,
    },
    async_trait,
};

use crate::config::{self, guild::set_config};

pub const CREATE: &str = "manager-ticket-create";
const CLOSE: &str = "manager-ticket-close";
const CLOSE_CONFIRMATION: &str = "manager-ticket-close-confirmation";
const CANCEL_CLOSE: &str = "manager-ticket-cancel-close";
const OPEN: &str = "manager-ticket-open";
const DELETE: &str = "manager-ticket-delete";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Some(guild_id) = interaction.guild_id() else {
            return;
        };

        let guild_config = config::guild::get_config(guild_id.get());

        if !guild_config.ticket_system {
            return;
        }

        let Some(component) = interaction.as_message_component() else {
            return;
        };
        let Some(member) = component.member.clone() else {
            return;
        };

        match component.data.custom_id.as_str() {
            CREATE => {
                self.create_ticket(ctx, component.clone(), member, guild_id)
                    .await
            }
            CLOSE => self.close_ticket(ctx, component.clone(), guild_id).await,
            CLOSE_CONFIRMATION => {
                self.confirm_close(ctx, component.clone(), member, guild_id)
                    .await
            }
            CANCEL_CLOSE => self.cancel_close(ctx, component.clone()).await,
            OPEN => {
                self.open_ticket(ctx, component.clone(), member, guild_id)
                    .await
            }
            DELETE => self.delete_ticket(ctx, component.clone(), guild_id).await,
            _ => {}
        }
    }
}

impl Handler {
    async fn create_ticket(
        &self,
        ctx: Context,
        component: serenity::all::ComponentInteraction,
        member: serenity::all::Member,
        guild_id: serenity::all::GuildId,
    ) {
        let mut guild_config = config::guild::get_config(guild_id.get());
        let category: ChannelId = guild_config.ticket_category.into();

        let ticket_number = guild_config.ticket_number + 1;
        let formatted = format!("ticket-{:04}", ticket_number);

        guild_config.ticket_number = ticket_number;
        if set_config(guild_id.get(), guild_config.clone()).is_err() {
            return;
        }

        let mut overwrites: Vec<PermissionOverwrite> = vec![
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(member.user.id),
            },
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
            },
        ];

        for role in guild_config.moderator_roles {
            overwrites.push(PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(role.into()),
            });
        }

        let Ok(channel) = guild_id
            .create_channel(
                &ctx.http,
                CreateChannel::new(formatted)
                    .category(category)
                    .permissions(overwrites),
            )
            .await
        else {
            return;
        };

        let embed = CreateEmbed::new()
            .title("Tickets")
            .description("Support will be with you shortly. To close this ticket react with 🔒")
            .footer(CreateEmbedFooter::new("Manager - Ticket System"))
            .color(Colour::DARKER_GREY);

        let _ = channel
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(format!("{}", member.mention()))
                    .add_embed(embed)
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new(CLOSE)
                            .label("Close")
                            .emoji(ReactionType::Unicode("🔒".to_string()))
                            .style(ButtonStyle::Secondary),
                    ])]),
            )
            .await;

        let _ = component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("✔ Ticket Created {}", channel.mention()))
                        .ephemeral(true),
                ),
            )
            .await;
    }

    async fn close_ticket(
        &self,
        ctx: Context,
        component: serenity::all::ComponentInteraction,
        _guild_id: serenity::all::GuildId,
    ) {
        let Ok(Channel::Guild(guild_channel)) = component.channel_id.to_channel(&ctx.http).await
        else {
            return;
        };

        if guild_channel.name.starts_with("closed-") {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("> **Warning:** ticket already closed")
                            .ephemeral(true),
                    ),
                )
                .await;
        } else {
            let _ = component.defer(&ctx.http).await;

            let _ = component
                .channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content("Are you sure you would like to close this ticket?")
                        .components(vec![CreateActionRow::Buttons(vec![
                            CreateButton::new(CLOSE_CONFIRMATION)
                                .label("Close")
                                .style(ButtonStyle::Danger),
                            CreateButton::new(CANCEL_CLOSE)
                                .label("Cancel")
                                .style(ButtonStyle::Secondary),
                        ])]),
                )
                .await;
        }
    }

    async fn confirm_close(
        &self,
        ctx: Context,
        component: serenity::all::ComponentInteraction,
        member: serenity::all::Member,
        guild_id: serenity::all::GuildId,
    ) {
        let _ = component.defer(&ctx.http).await;
        let _ = component.message.delete(&ctx.http).await;
        tokio::time::sleep(Duration::from_millis(500)).await;

        let channel_id = component.channel_id;

        let _ = channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().add_embed(
                    CreateEmbed::new()
                        .description(format!("Ticket Closed by {}", member.mention()))
                        .color(Colour::DARK_GOLD),
                ),
            )
            .await;

        let Some(Channel::Guild(mut guild_channel)) = channel_id.to_channel(&ctx.http).await.ok()
        else {
            return;
        };
        let suffix = guild_channel
            .name
            .split('-')
            .nth(1)
            .unwrap_or("0000")
            .to_string();

        let _ = guild_channel
            .edit(
                &ctx.http,
                EditChannel::new().name(format!("closed-{suffix}")),
            )
            .await;

        let overwrites = guild_channel.permission_overwrites.clone();
        for overwrite in overwrites {
            let PermissionOverwriteType::Member(user_id) = overwrite.kind else {
                continue;
            };

            if user_id.get() == guild_id.to_guild_cached(&ctx.cache).unwrap().owner_id.get() {
                continue;
            }
            let is_bot = user_id.to_user(&ctx.http).await.map_or(true, |u| u.bot);
            if is_bot {
                continue;
            }

            let _ = guild_channel
                .create_permission(
                    &ctx.http,
                    PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: Permissions::VIEW_CHANNEL,
                        kind: PermissionOverwriteType::Member(user_id),
                    },
                )
                .await;
        }

        let controls = CreateEmbed::new()
            .description("```Support team ticket controls```")
            .color(Colour::DARK_GREY);

        let _ = channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().add_embed(controls).components(vec![
                    CreateActionRow::Buttons(vec![
                        CreateButton::new(OPEN)
                            .label("Open")
                            .emoji(ReactionType::Unicode("🔓".to_string()))
                            .style(ButtonStyle::Secondary),
                        CreateButton::new(DELETE)
                            .label("Delete")
                            .emoji(ReactionType::Unicode("⛔".to_string()))
                            .style(ButtonStyle::Secondary),
                    ]),
                ]),
            )
            .await;
    }

    async fn cancel_close(&self, ctx: Context, component: serenity::all::ComponentInteraction) {
        let _ = component.defer(&ctx.http).await;
        let _ = component.message.delete(&ctx.http).await;
    }

    async fn open_ticket(
        &self,
        ctx: Context,
        component: serenity::all::ComponentInteraction,
        member: serenity::all::Member,
        guild_id: serenity::all::GuildId,
    ) {
        let _ = component.defer(&ctx.http).await;
        let channel_id = component.channel_id;

        let Ok(Channel::Guild(mut guild_channel)) = channel_id.to_channel(&ctx.http).await else {
            return;
        };
        let suffix = guild_channel
            .name
            .split('-')
            .nth(1)
            .unwrap_or("0000")
            .to_string();

        let _ = guild_channel
            .edit(
                &ctx.http,
                EditChannel::new().name(format!("ticket-{suffix}")),
            )
            .await;

        let overwrites = guild_channel.permission_overwrites.clone();
        for overwrite in overwrites {
            if let PermissionOverwriteType::Member(user_id) = &overwrite.kind {
                if user_id.get() == guild_id.to_guild_cached(&ctx.cache).unwrap().owner_id.get() {
                    continue;
                }
                let is_bot = user_id.to_user(&ctx.http).await.map_or(true, |u| u.bot);
                if is_bot {
                    continue;
                }

                let _ = guild_channel
                    .create_permission(
                        &ctx.http,
                        PermissionOverwrite {
                            allow: Permissions::VIEW_CHANNEL,
                            deny: Permissions::empty(),
                            kind: PermissionOverwriteType::Member(*user_id),
                        },
                    )
                    .await;
            }
        }

        if let Ok(messages) = channel_id
            .messages(&ctx.http, GetMessages::new().limit(10))
            .await
        {
            for message in messages {
                let is_controls = message
                    .embeds
                    .first()
                    .and_then(|e| e.description.as_deref())
                    .is_some_and(|d| d == "```Support team ticket controls```");

                if is_controls {
                    let _ = message.delete(&ctx.http).await;
                    break;
                }
            }
        }

        let _ = channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().add_embed(
                    CreateEmbed::new()
                        .description(format!("Ticket Opened by {}", member.mention()))
                        .color(Colour::DARK_GREEN),
                ),
            )
            .await;
    }

    async fn delete_ticket(
        &self,
        ctx: Context,
        component: serenity::all::ComponentInteraction,
        _guild_id: serenity::all::GuildId,
    ) {
        let Ok(Channel::Guild(guild_channel)) = component.channel_id.to_channel(&ctx.http).await
        else {
            return;
        };

        if !guild_channel.name.starts_with("closed-") {
            return;
        }

        let _ = component.defer(&ctx.http).await;

        let Ok(_msg) = component
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().add_embed(
                    CreateEmbed::new()
                        .description("Ticket will be deleted in a few seconds")
                        .color(Colour::RED),
                ),
            )
            .await
        else {
            return;
        };

        tokio::time::sleep(Duration::from_secs(5)).await;
        let _ = component.channel_id.delete(&ctx.http).await;
    }
}
