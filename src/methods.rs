use std::time::Duration;

use axum::{extract::State, headers::{authorization::Bearer, Authorization}};
use bb8::{PooledConnection, ManageConnection};
use chrono::prelude::*;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use serde::Deserialize;
use serde_json::{Deserializer, Value};

use crate::{config::AppError, Pool, handler::users::Claims};

pub fn now()->String{
    Local::now().timestamp().to_string()
}

pub fn my_decode(auth:Authorization<Bearer>)->Result<Claims,AppError>{
    decode::<Claims>(&auth.token(), &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).map_err(|e| AppError::err(500,e.to_string())).and_then(|e| Ok(e.claims))
}


pub async fn get_connection(pool:&Pool)->Result<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,AppError>{
    pool.get().await.map_err(|e| AppError::err(500,e.to_string()))
}

//定时发送消息
pub async fn send_msg(start_time:i64){
    // tokio::time::sleep(Duration::from_secs(1));
  
    let hint_time=start_time-now().parse::<i64>().unwrap()-300;
    tokio::time::sleep(Duration::from_secs(hint_time as u64)).await;
    println!("hint_time:{:?}",hint_time);
    
    //发送消息
    println!("我已经成功发送消息");
}