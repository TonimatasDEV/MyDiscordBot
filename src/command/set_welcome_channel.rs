use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse,
    CreateInteractionResponseMessage, InteractionContext, Permissions, ResolvedValue,
};

use crate::config::guild::{get_config, set_config};

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    let guild_id = interaction.guild_id.unwrap().get();
    let mut config = get_config(guild_id);

    let options = interaction.data.options();
    let option = options.first().unwrap();
    let channel = match option.value {
        ResolvedValue::Channel(channel) => channel,
        _ => unreachable!(),
    };

    config.welcome_message_id = channel.id.get();

    match set_config(guild_id, config) {
        Ok(_) => {
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
    CreateCommand::new("set-welcome-channel")
        .add_context(InteractionContext::Guild)
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Channel,
                "channel",
                "Select a channel.",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("Change channel where welcome messages will be send.")
}
