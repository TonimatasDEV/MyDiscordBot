use std::time::Instant;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse,
};

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    let data = CreateInteractionResponseMessage::new().content("Pong!");
    let builder = CreateInteractionResponse::Message(data);
    let start = Instant::now();
    if let Ok(_) = interaction.create_response(&ctx.http, builder).await {
        let elapsed = start.elapsed().as_millis();
        let elapsed_message =
            EditInteractionResponse::new().content(format!("Pong! {}ms", elapsed));
        let _ = interaction.edit_response(&ctx.http, elapsed_message).await;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Ping! Pong!")
}
