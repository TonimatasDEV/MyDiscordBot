use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::config::guild::{get_config, set_config};

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    let guild_id = interaction.guild_id.unwrap().get();
    let mut config = get_config(guild_id);

    config.ticket_number = 0;

    match set_config(guild_id, config) {
        Ok(_) => {
            let data = CreateInteractionResponseMessage::new()
                .content("Tickets number has been reset correctly.")
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
    CreateCommand::new("reset-tickets-number").description("Reset the tickets number.")
}
