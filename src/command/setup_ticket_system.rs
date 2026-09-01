use serenity::all::{
    ButtonStyle, Colour, CommandInteraction, Context, CreateActionRow, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, InteractionContext, Permissions, ReactionType,
    ResolvedValue,
};

use crate::{
    config::guild::{get_config, set_config},
    listener::ticket_system::CREATE,
};

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    let guild_id = interaction.guild_id.unwrap().get();
    let mut config = get_config(guild_id);

    let options = interaction.data.options();
    let first_option = options.first().unwrap();
    let channel = match first_option.value {
        ResolvedValue::Channel(channel) => channel,
        _ => unreachable!(),
    };

    let second_option = options.get(1).unwrap();
    let category_id = match second_option.value {
        ResolvedValue::Channel(category_id) => category_id,
        _ => unreachable!(),
    };

    config.ticket_channel = channel.id.get();
    config.ticket_category = category_id.id.get();
    config.ticket_system = true;
    config.ticket_number = 0;

    match set_config(guild_id, config) {
        Ok(_) => {
            let embed = CreateEmbed::new()
                .title("Tickets")
                .description("To create a ticket and request support, click 📩")
                .footer(CreateEmbedFooter::new("Manager - Ticket System"))
                .color(Colour::DARK_GREEN);

            let _ = channel
                .id
                .send_message(
                    &ctx.http,
                    CreateMessage::new().add_embed(embed).components(vec![
                        CreateActionRow::Buttons(vec![
                            CreateButton::new(CREATE)
                                .label("Create ticket")
                                .emoji(ReactionType::Unicode("📩".to_string()))
                                .style(ButtonStyle::Secondary),
                        ]),
                    ]),
                )
                .await;

            let data = CreateInteractionResponseMessage::new()
                .content("New channel has been set to send welcome messages.")
                .ephemeral(true);
            let builder = CreateInteractionResponse::Message(data);
            let _ = interaction.create_response(&ctx.http, builder).await;
        }
        Err(_) => {
            let data = CreateInteractionResponseMessage::new()
                .content("Error saving configuration.")
                .ephemeral(true);
            let builder = CreateInteractionResponse::Message(data);
            let _ = interaction.create_response(&ctx.http, builder).await;
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup-ticket-system")
        .add_context(InteractionContext::Guild)
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Channel,
                "channel",
                "Select a channel to send the ticket panel message.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Channel,
                "category",
                "Add the tickets category id.",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("Setup the ticket system.")
}
