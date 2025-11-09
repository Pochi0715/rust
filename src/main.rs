// renderの場合これも消す
// use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use std::env;

# [derive(Debug)]
pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

# [poise::command(slash_command, prefix_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello!! this bot makes language is **RUST!!**").await?;
    Ok(())
}

# [poise::command(slash_command, prefix_command)]
async fn bot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This bot is **RUSTテストbot**").await?;
    Ok(())
}

# [poise::command(slash_command, prefix_command)]
async fn name(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author();
    let name = &user.name;
    let user_id = user.id.get();
    let response = format!(
        "Hi!! **{}** \nyour Discord ID is  `{}` ",
        name,
        user_id
    );
    ctx.say(response).await?;
    Ok(())
}

# [poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping_duration = ctx.ping().await;

    let ping_ms = ping_duration.as_millis();

    let response = format!(
        "pong!! ping is **{}ms**",
        ping_ms
    );

    ctx.say(response).await?;
    Ok(())
}

# [poise::command(slash_command, prefix_command, guild_only)]
async fn role(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("Commands can only be used in server :(").await?;
            return Ok(());
        }
    };
    
    let member = match ctx.author_member().await {
        Some(member) => member,
        None => {
            ctx.say("Failure to acquire member information. ").await?;
            return Ok(());
        }
    };

    let role_map = ctx.serenity_context().cache.guild(guild_id)
        .map(|guild| guild.roles.clone())
        .unwrap_or_default();

    let role_names: Vec<String> = member.roles
        .iter()
        .filter_map(|role_id| {
            role_map.get(role_id).map(|role| role.name.clone())
        })
        .collect();
    
    let user_name = ctx.author().name.clone();
    
    if role_names.len() <= 1 { 
        ctx.say(format!(
            "Hi **{}** not server roles ㅠㅠ",
            user_name
        )).await?;
    } else {
        let role_list = role_names.join(", ");
        let response = format!(
            "Hi! **{}** ! your roles below:\n{}",
            user_name,
            role_list
        );
        ctx.say(response).await?;
    }

    Ok(())
}

# [tokio::main]
async fn main() {
    // renderでは不要実機環境では必須
    // dotenv().expect("no settings '.env'"); 
    
    let token = env::var("DISCORD_TOKEN")
        .expect("no settings'DISCORD_TOKEN' ");
    
    let options = poise::FrameworkOptions {
        commands: vec![hello(), bot(), name(), ping(), role()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };
    
    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("not conect discord");

    if let Err(why) = client.start().await {
        eprintln!("client error: {:?}", why);
    }
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            eprintln!("Error in command {}: {:?}", ctx.command().name, error);
        }
        _ => eprintln!("error: {:?}", error),
    }
}
