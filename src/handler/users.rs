use std::sync::{Arc, Mutex};

use axum::{
    extract::{ State, Path},
    response::{Json, IntoResponse},
    routing::{get, post,delete},
    Router,
};
use crate::{Pool, methods::get_connection, config::Share};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use serde_json::json;
use jsonwebtoken::{encode, decode,Header,Algorithm,EncodingKey};
use crate::schema::users::{self,username,password,phone};
use crate::models::users::{User,NewUser,Login};
use crate::config::{AppError};

// use crate::*;
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims{
   pub uuid:String,
   pub username:String,
   pub password:String,
   pub exp:usize
}
//创建用户
pub async fn create_user(State(share): State<Arc<Share>>,Json(user): Json<NewUser>)->Result<impl IntoResponse,AppError>{
    println!("我收到请求了");
    let mut conn = share.pool.get().await.map_err(|e| AppError::err(500,e.to_string()))?;
    let a=users::table.filter(username.eq(&user.username)).load::<User>(&mut conn).await.map_err(|err| AppError::err(500,err.to_string()))?;
    println!("查到的数量:{:?}",a.len());
    if a.len()==0{
        diesel::insert_into(users::table).values(user).execute(&mut conn).await.map_err(|err| AppError::err(500,err.to_string()))?;
       
        Ok(Json(json!({
            "code":200,
            "msg":"创建成功"
        })))
    }else{
       return Err(AppError::err(500,"此用户已经存在".to_string()));
    }
}

//登录
pub async fn login(State(share): State<Arc<Share>>,Json(user):Json<Login>)->Result<impl IntoResponse,AppError>{
    let mut conn =get_connection(&share.pool).await?;
    let row=users::table.filter(username.eq(&user.username)).filter(password.eq(&user.password)).load::<User>(&mut conn).await.map_err(|e| AppError::err(500,e.to_string()))?;
    println!("row:{:?}",row);
    if row.len()>0{
        let my_claims=Claims{
            uuid:row[0].uuid.clone(),
            username:user.username,
            password:user.password,
            exp:1000000000000,
        };
        let token=encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).map_err(|e| AppError::err(500,"加密失败".to_string()))?;
        return Ok(Json(json!({
            "code":200,
            "msg":"登录成功",
            "data":token
        })));
    }else{
        return Err(AppError::err(500,"账号或者密码错误".to_string()));
    }

}

//注册
pub async fn register(State(share): State<Arc<Share>>,Json(mut user): Json<NewUser>)->Result<impl IntoResponse,AppError>{
    let mut conn = get_connection(&share.pool).await?;
    let user_exist=users::table.filter(username.eq(&user.username)).execute(&mut conn).await.map_err(|err| AppError::err(500,err.to_string()))?;
    user.uuid=Some(uuid::Uuid::new_v4().to_string());
    if user_exist<1{
        diesel::insert_into(users::table).values(user).execute(&mut conn).await.map_err(|err| AppError::err(500,err.to_string()))?;
        Ok(Json(json!({
            "code":200,
            "msg":"注册成功",
        })))
    }else{
        
        Err(AppError::err(500, "用户已经存在".to_string()) )
    }
}
