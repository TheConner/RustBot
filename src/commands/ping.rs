///
/// Boring ping command for sanity testing
/// 
use serenity::framework::standard::macros::{command};
use serenity::model::prelude::{Message};
use serenity::prelude::{Context};
use serenity::framework::standard::{CommandResult};

use rustbot::constants::{CHECK_MARK_EMOJI};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, CHECK_MARK_EMOJI).await?;
    msg.channel_id.say(&ctx.http, "PONG").await?;
    Ok(())
}