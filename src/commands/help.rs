/// Bot help
use serenity::framework::standard::macros::{command};
use serenity::model::prelude::{Message};
use serenity::prelude::{Context};
use serenity::framework::standard::{CommandResult};
use rustbot::util::template_reader::{template_reader};

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let response = template_reader("help_general");
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}