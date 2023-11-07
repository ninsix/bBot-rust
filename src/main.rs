use poise::serenity_prelude as serenity;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod converter;
use converter::convert::*;
/// Displays your or another user's account creation date


/// Copy message for testing
#[poise::command(slash_command, prefix_command)]
async fn copy(
    ctx: Context<'_>,
    #[description = "Copy a message"] custom_string: String,
) -> Result<(), Error> {

    let response = custom_string;
    
    ctx.say(response).await?;
    Ok(())
}

/// Convert text to bB
#[poise::command(slash_command, prefix_command)]
async fn ttb(
    ctx: Context<'_>,
    #[description = "Text to convert"] str: String,
) -> Result<(), Error> {

    let response = format(&text_to_bb(&str));
    
    ctx.say(response).await?;
    Ok(())
}

/// Convert bB to text
#[poise::command(slash_command, prefix_command)]
async fn btt(
    ctx: Context<'_>,
    #[description = "Text to convert"] str: String,
) -> Result<(), Error> {

    let response = format(&bb_to_text(&str));
    
    ctx.say(response).await?;
    Ok(())
}

/// Convert text to binary
#[poise::command(slash_command, prefix_command)]
async fn tt_b(
    ctx: Context<'_>,
    #[description = "Text to convert"] str: String,
) -> Result<(), Error> {

    let response = format(&text_to_binary(&str));
    
    ctx.say(response).await?;
    Ok(())
}

/// Convert binary to text
#[poise::command(slash_command, prefix_command)]
async fn b_tt(
    ctx: Context<'_>,
    #[description = "Text to convert"] str: String,
) -> Result<(), Error> {

    let response = format(&binary_to_text(&str));
    
    ctx.say(response).await?;
    Ok(())
}

/// Bot information
#[poise::command(slash_command, prefix_command)]
async fn info(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = format("bBot (Poise) 1.0\nMade with Rust uwu");
    ctx.say(response).await?;
    Ok(())
}


#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![copy(), ttb(), btt(), info(), b_tt(), tt_b()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
