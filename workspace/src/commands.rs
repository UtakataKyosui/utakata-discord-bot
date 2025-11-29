use std::{str, time::Duration};
use sea_orm::DatabaseConnection;
use serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateMessage};
use tokio::{sync::Mutex, time};

use crate::utils::{user_id_add_point, user_id_get_point, user_id_sub_point};

#[derive(Debug)]
pub struct Data {
    // pub points: Mutex<u32>
    pub db: Mutex<DatabaseConnection>
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

// play slot game
#[poise::command(slash_command)]
pub async fn slot_play(ctx: Context<'_>) -> Result<(),Error> {
    use rand::seq::SliceRandom;
    
    ctx.defer().await?;

    let current_user = ctx.author();
    let db = &ctx.data().db.lock().await;
    let point = user_id_get_point(current_user, db).await;

    if point < 10 {
        ctx.say("ポイントが足りません！貯めてから再チャレンジしてね！").await?;
        return Ok(())
    }

    if let Err(why) = user_id_sub_point(current_user, db, 10).await {
        ctx.say(why).await?;
        return Ok(())
    }
    let mut slot_emojis:Vec<&str> = emojis::iter()
        .map(|e| e.as_str())
        .take(10)
        .cycle()
        .take(10 * 3)
        .collect();
    slot_emojis.shuffle(&mut rand::thread_rng());

    let [first,second,third] = slot_emojis[..3]
        .try_into()
        .unwrap();

    let slot_reach_first_result = first == second;
    let slot_reach_second_result = second == third;

    if slot_reach_first_result {
        ctx.say(format!("{} {} リーチ！ 【リーチボーナス +5pt】",first,second)).await?;
        user_id_add_point(current_user, db, 5).await;
    }

    if slot_reach_second_result {
        ctx.say(format!("{} {} リーチ！ 【リーチボーナス +5pt】 ",second,third)).await?;
        user_id_add_point(current_user, db, 5).await;
    }

    time::sleep(Duration::new(3, 0)).await;

    let slot_result = slot_reach_first_result && slot_reach_first_result;
    let result = if slot_result {
        format!("{} {} {} 揃いました！嬉しいね！",first,second,third)
    }else {
        format!("{} {} {} 揃わなかった...おつらいね...",first,second,third)
    };

    if slot_result {
        user_id_add_point(current_user, db, 20).await;
    }
    ctx.say(result).await?;

    Ok(())
}

// show your points
#[poise::command(slash_command)]
pub async fn show_point(
    ctx: Context<'_>
) -> Result<(),Error> { 
    let current_user = ctx.author();
    let db = ctx.data().db.lock().await;
    ctx.say(format!(
        "{:#?}",
        user_id_get_point(current_user, &*db).await
    )).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn developer_access(ctx: Context<'_>) -> Result<(),Error> {
    ctx.defer().await?;
    if let Err(why) = ctx.channel_id().send_message(
        &ctx.http(),
        CreateMessage::new().add_embed(
            CreateEmbed::new()
                .author(CreateEmbedAuthor::new("泡沫 京水"))
                .url("https://x.com/utakatakyosui")
                .description("プログラミングﾁｮｯﾄﾃﾞｷﾙ大学生")
                .image("https://pbs.twimg.com/profile_images/1861649062606577665/GHbnyWIG_400x400.jpg")
            )
    )
    .await {
        eprintln!("エラー: {:?}",why);
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn gift_point(
    ctx: Context<'_>,
    #[description = "ポイントを追加するユーザー"] user: poise::serenity_prelude::User,
    #[description = "追加するポイント数"] add_point: u32
) -> Result<(),Error> {
    let db = ctx.data().db.lock().await;
    user_id_add_point(&user, &*db, add_point).await;
    if let Err(why) = user_id_sub_point(ctx.author(), &*db, add_point).await {
        ctx.say(why).await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn check_emoji(
    ctx: Context<'_>,
) -> Result<(),Error> {
    let mut emojis_str = String::new();
    for emoji in emojis::iter() {
        emojis_str.push_str(emoji.as_str());
    }
    ctx.say(emojis_str).await?;
    Ok(())
}