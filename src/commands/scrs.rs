use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "sdvx chart randomize serection!"]
async fn sc(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, format!("{} Play this chart! ",msg.author.mention()))
        .await?;
    Ok(())
}

// #[command]
// #[description = "hello"]
// async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.replay(&ctx.http, "hello, {}!", msg.author).await?;
//     Ok(())
// }
