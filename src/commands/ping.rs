///
/// Boring ping command for sanity testing
///
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use rustbot::constants::CHECK_MARK_EMOJI;

use tracing::{info};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    info!("PING command from {}", msg.author.name);
    msg.react(&ctx.http, CHECK_MARK_EMOJI).await?;
    msg.channel_id.say(&ctx.http, "PONG").await?;
    Ok(())
}
