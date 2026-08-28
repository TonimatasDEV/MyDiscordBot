use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse,
    CreateInteractionResponseMessage, InteractionContext, Permissions, ResolvedValue,
};

use crate::config::server::{get_config, set_config};

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    let guild_id = interaction.guild_id.unwrap().get();
    let mut config = get_config(guild_id);

    let options = interaction.data.options();
    let option = options.first().unwrap();
    let role = match option.value {
        ResolvedValue::Role(role) => role,
        _ => unreachable!(),
    };

    config.auto_role_id = role.id.get();

    match set_config(guild_id, config) {
        Ok(_) => {
            let data = CreateInteractionResponseMessage::new()
                .content("The new role has been set to be added automatically.")
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
    CreateCommand::new("set-auto-role")
        .add_context(InteractionContext::Guild)
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Role,
                "role",
                "Role to set automatically when member join to the guild.",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("Change role to be set automatically when a member join.")
}
