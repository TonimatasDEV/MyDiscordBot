use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse,
    CreateInteractionResponseMessage, InteractionContext, Permissions, ResolvedValue,
};

use crate::config::guild::{get_config, set_config};

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    let guild_id = interaction.guild_id.unwrap().get();
    let mut config = get_config(guild_id);

    let options = interaction.data.options();
    let system_option = options.first().unwrap();
    let system = match system_option.value {
        ResolvedValue::String(system) => system,
        _ => unreachable!(),
    };

    let enabled_option = &options.get(1).unwrap();
    let enabled = match enabled_option.value {
        ResolvedValue::Boolean(enabled) => enabled,
        _ => unreachable!(),
    };

    match system {
        "auto-role" => config.auto_role_system = enabled,
        "welcome-message" => config.welcome_message_system = enabled,
        _ => return,
    }

    let enabled_message = if enabled { "enabled" } else { "disabled" };

    match set_config(guild_id, config) {
        Ok(_) => {
            let data = CreateInteractionResponseMessage::new()
                .content(format!("System {} has been {}.", system, enabled_message))
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
    CreateCommand::new("system-config")
        .add_context(InteractionContext::Guild)
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "system",
                "Select the system to enable or disable.",
            )
            .add_string_choice("auto-role", "auto-role")
            .add_string_choice("welcome-message", "welcome-message")
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Boolean,
                "enabled",
                "Enable or disable.",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("Change channel where welcome messages will be send.")
}
