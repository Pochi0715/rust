// renderの場合これも消す
// use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use std::env;

#[derive(Debug)]
pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello!! this bot makes language is **RUST!!**").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn bot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This bot is **RUSTテストbot**").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
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

#[poise::command(slash_command, prefix_command)]
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

#[poise::command(slash_command, prefix_command, guild_only)]
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

#[poise::command(slash_command, prefix_command, guild_only)]
async fn verify(
    ctx: Context<'_>,
    #[description = "Role to assign when verified"] role: serenity::Role,
) -> Result<(), Error> {
    let role_id = role.id;

    let button = serenity::CreateButton::new(format!("verify_{}", role_id))
        .label("Verify")
        .style(serenity::ButtonStyle::Success);

    let components = vec![serenity::CreateActionRow::Buttons(vec![button])];

    let builder = poise::CreateReply::default()
        .content(format!("Click the button below to get the **{}** role!", role.name))
        .components(components);

    ctx.send(builder).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // renderでは不要実機環境では必須
    // dotenv().expect("no settings '.env'"); 
    
    let token = env::var("DISCORD_TOKEN")
        .expect("no settings'DISCORD_TOKEN' ");
    
    let options = poise::FrameworkOptions {
        commands: vec![hello(), bot(), name(), ping(), role(), verify()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("&".into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        event_handler: |ctx, event, _framework, _data| {
            Box::pin(async move {
                if let poise::serenity_prelude::FullEvent::InteractionCreate { interaction } = event {
                    if let Some(interaction) = interaction.as_message_component() {
                        if interaction.data.custom_id.starts_with("verify_") {
                            let role_id_str = interaction.data.custom_id.strip_prefix("verify_").unwrap();
                            if let Ok(role_id_u64) = role_id_str.parse::<u64>() {
                                let role_id = serenity::RoleId::new(role_id_u64);
                                
                                if let Some(guild_id) = interaction.guild_id {
                                    if let Some(member) = &interaction.member {
                                        match ctx.http.add_member_role(
                                            guild_id,
                                            member.user.id,
                                            role_id,
                                            Some("Verified via button"),
                                        ).await {
                                            Ok(_) => {
                                                let response = serenity::CreateInteractionResponse::Message(
                                                    serenity::CreateInteractionResponseMessage::new()
                                                        .content("✅ You have been verified!")
                                                        .ephemeral(true)
                                                );
                                                let _ = interaction.create_response(&ctx.http, response).await;
                                            }
                                            Err(e) => {
                                                let response = serenity::CreateInteractionResponse::Message(
                                                    serenity::CreateInteractionResponseMessage::new()
                                                        .content(format!("❌ Failed to assign role: {}", e))
                                                        .ephemeral(true)
                                                );
                                                let _ = interaction.create_response(&ctx.http, response).await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(())
            })
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
