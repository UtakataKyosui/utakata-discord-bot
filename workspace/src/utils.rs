use sea_orm::DatabaseConnection;
use crate::entities::record::{Column,ActiveModel};
use crate::entities::prelude::Record;
use sea_orm::{EntityTrait,QueryFilter,ColumnTrait,ActiveValue,ActiveModelTrait,Set};

pub async fn user_id_get_point(
    user: &poise::serenity_prelude::User,
    db: &DatabaseConnection
) -> u32 {
    use std::string::ToString;
    let current_user_id = user.id.get().to_string();
    let exist_data = Record::find().filter(
        Column::Key.eq(&current_user_id)
    ).one(db).await
    .expect("Database Connection Error");

    if let Some(record) = exist_data {
        return record.value as u32;
    } else {
        let new_record = ActiveModel {
            key: ActiveValue::Set(current_user_id),
            value: ActiveValue::Set(500 as i32)
        };
        let record = new_record.insert(db).await.expect("New Record Data Insert Error");
        return record.value as u32
    }
}

pub async fn user_id_add_point(
    user: &poise::serenity_prelude::User,
    db: &DatabaseConnection,
    add_point: u32
) {
    use std::string::ToString;
    let current_user_id = user.id.get().to_string();

    let exist_data = Record::find().filter(
        Column::Key.eq(&current_user_id)
    ).one(db).await
    .expect("Database Connection Error");

    if let Some(record) = exist_data {
        let mut target: ActiveModel = record.into();
        let updated_value = target.value.unwrap() + add_point as i32;
        target.value = Set(updated_value);
        target.update(db).await.expect("Update Record Error");
    }
}

pub async fn user_id_sub_point(
    user: &poise::serenity_prelude::User,
    db: &DatabaseConnection,
    sub_point: u32
) -> Result<(),std::string::String>{
    use std::string::ToString;
    let current_user_id = user.id.get().to_string();
    let exist_data = Record::find().filter(
        Column::Key.eq(&current_user_id)
    ).one(db).await
    .expect("Database Connection Error");

    if let Some(record) = exist_data {
        let mut target: ActiveModel = record.into();
        let before_value = target.value.unwrap();
        if before_value - sub_point as i32 <= 0 {
            eprintln!("ポイントが足りません");
            return Err(String::from("所持ポイント以上の減算が行われようとしました。"));
        }

        let updated_value = before_value - sub_point as i32;
        target.value = Set(updated_value);
        target.update(db).await.expect("Update Record Error");
    }

    Ok(())
}

#[allow(unused)]
pub async fn user_to_user_point(    
    user: &poise::serenity_prelude::User,
    db: &DatabaseConnection,
    to_user: &poise::serenity_prelude::User,
    to_user_points: u32
) {
    user_id_sub_point(user, &db, to_user_points).await.unwrap();
    user_id_add_point(to_user, &db, to_user_points).await;
}
