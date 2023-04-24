use std::{sync::{Arc, Mutex}, io::{BufReader, Read, BufWriter}, fs::File};

use axum::{extract::{State,Json, Path,Multipart}, response::IntoResponse, TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{Header, encode, decode, DecodingKey, Validation};
use serde_json::json;

use crate::{Pool, config::{AppError, Share}, schema::{users::{username, self}, todo_item::{self, owner, id, title, content, status, start_time}}, models::{todo_item::{TodoItem, UpdateTodoItem, QueryCondition, Notify, CancelNotify}, users::User}, methods::{now, my_decode, get_connection, send_msg}};
use crate::models::todo_item::{NewTodoItem};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use diesel::prelude::*;
use crate::models;
use super::users::Claims;
use std::io::Write;
//创建代办列表
pub async fn create(TypedHeader(auth): TypedHeader<Authorization<Bearer>>,State(share): State<Arc<Share>>,Json(mut newItem): Json<NewTodoItem>, )->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&share.pool).await?;
    let claims = my_decode(auth)?;
    newItem.owner=Some(claims.uuid);
    newItem.create_at=Some(now());  
    println!("修改后的newItem:{:?}",newItem);
    //添加代办事项
    diesel::insert_into(todo_item::table).values(newItem).execute(&mut conn).await.map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"创建成功",
    })))
}

//用户查看所有代办事项
pub async fn list(TypedHeader(auth): TypedHeader<Authorization<Bearer>>,State(share): State<Arc<Share>>,Json(condition):Json<QueryCondition>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&share.pool).await?;
    let claims = my_decode(auth)?;
    let user=users::table.filter(username.eq(claims.username)).first::<User>(&mut conn).await.map_err(|e| AppError::err(500,e.to_string()))?;
    println!("uuuuu:{:?}",user);
    let mut query=todo_item::table.into_boxed();
    if let Some(value)=condition.status{
        query=query.filter(status.eq(value))
    }
    let lists=query.load::<TodoItem>(&mut conn).await.map_err(|e| AppError::err(500,e.to_string()))?; 

    Ok(Json(json!({
        "code":200,
        "msg":"请求成功",
        "data":{
            "lists":lists
        }
    })))
}

//更新代办事项
pub async fn update(State(share): State<Arc<Share>>,Json(updateItem):Json<UpdateTodoItem>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&share.pool).await?;
    let row=diesel::update(todo_item::table.filter(id.eq(updateItem.id))).set((title.eq(updateItem.title),content.eq(updateItem.content),status.eq(updateItem.status),start_time.eq(updateItem.start_time))).execute(&mut conn).await.map_err(|e| AppError::err(500,e.to_string()))?;
    if row>0{
        Ok(Json(json!({
            "code":200,
            "msg":"更新成功"
        })))
    }else{
        Err(AppError::err(500,"待办事项不存在".to_string()))
    }
}

//删除代办事项
pub async fn delete(State(share): State<Arc<Share>>,Path(i):Path<i32>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&share.pool).await?;
    let row=diesel::delete(todo_item::table.filter(id.eq(i)))
                        .execute(&mut conn)
                        .await
                        .map_err(|e| AppError::err(500,e.to_string()))?;
    if row>0{
        println!("iiii:1");
        Ok(Json(json!({
            "code":200,
            "msg":"删除成功"
        })))
    }else{
        println!("iiii:2");
        Err(AppError::err(500,"没有找到对应的id".to_string()))
    }
}

//开启消息通知
pub async fn notify(State(share): State<Arc<Share>>,Json(info):Json<Notify>)->Result<impl IntoResponse,AppError>{
    // let mut conn=get_connection(&share.pool).await?;

    let h=tokio::spawn(async move{
        send_msg(info.start_time).await;
    });
    // TASK.insert(info.id,h);
    share.tasks.lock().unwrap().insert(info.id, h);
    Ok(Json(json!({
        "code":200,
        "msg":"添加消息通知成功"
    })))
}

//取消消息通知
pub async fn cancel_notify(State(share): State<Arc<Share>>,Json(info):Json<CancelNotify>)->Result<impl IntoResponse,AppError>{
    // let mut conn=get_connection(&share.pool).await?;

    // TASK.insert(info.id,h);
    share.tasks.lock().unwrap().get(&info.id).unwrap().abort();
    Ok(Json(json!({
        "code":200,
        "msg":"取消消息通知成功"
    })))
}

//给item上传图片
pub async fn accept_imgs(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let f=File::create(format!("./src/imgs/{}",&file_name)).expect("创建文件失败");
        let mut buffer=BufWriter::new(f);
        // tokio::fs::write(format!("./src/imgs/{}",&file_name),&data).await.unwrap();
        buffer.write(&data).expect("写入失败");
        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }
}
