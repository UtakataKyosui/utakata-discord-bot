use chrono::{DateTime,Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection,EntityTrait, Set};
use serde::{Deserialize,Serialize};
use apalis::prelude::*;
use crate::entities::{prelude::Record, record::ActiveModel};


#[derive(Default,Debug, Deserialize, Serialize, Clone)]
pub struct DairyCharger(DateTime<Utc>);

impl From<DateTime<Utc>> for DairyCharger {
    fn from(date: DateTime<Utc>) -> Self {
        DairyCharger(date)
    }
}

pub async fn dairy_charge(_job: DairyCharger,data: Data<DatabaseConnection>) {
    println!("Dairy charge job Start!");
    let all_user = Record::find()
        .all(&*data)
        .await
        .expect("Database Connection Error");
    for user in all_user {
        let mut target: ActiveModel = user.into();
        let updated_value = target.value.unwrap() + 500 as i32;
        target.value = Set(updated_value);
        target.update(&*data).await.expect("Update Record Error");
    }
    println!("Dairy charge job End!");
}