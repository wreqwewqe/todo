
use diesel::prelude::*;

use crate::schema::todo_item;

#[derive(serde::Serialize,serde::Deserialize, Queryable,Debug)]
pub struct TodoItem{
    pub id:i32,
    pub owner:String,
    pub title:String,
    pub content:String,
    pub status:bool,
    pub create_at:String,
    pub start_time:i64
}
#[derive(serde::Deserialize,serde::Serialize,Queryable,Debug,Insertable)]
#[diesel(table_name = todo_item)]
pub struct NewTodoItem{
    pub owner:Option<String>,
    pub title:String,
    pub content:String,
    pub status:Option<bool>,
    pub create_at:Option<String>,
    pub start_time:i64,
}


#[derive(Debug,serde::Deserialize,Insertable)]
#[diesel(table_name = todo_item)]
pub struct UpdateTodoItem{
    pub id:i32,
    pub title:String,
    pub content:String,
    pub status:bool,
    pub start_time:i64
}

//结果筛选
#[derive(Debug,Queryable,serde::Deserialize)]
pub struct QueryCondition{
    pub status:Option<bool>,
}

//开启通知服务
#[derive(Debug,Queryable,serde::Deserialize)]
pub struct Notify{
    pub id:i32,
    pub phone:String,
    pub title:String,
    pub content:String,
    pub start_time:i64
}

//取消通知服务
#[derive(Debug,Queryable,serde::Deserialize)]
pub struct CancelNotify{
    pub id:i32,
}