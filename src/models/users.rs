//用户表
use diesel::prelude::*;

use crate::schema::users;
#[derive(serde::Serialize,serde::Deserialize, Queryable,Debug)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub phone: Option<String>,
}

#[derive(serde::Deserialize,Queryable,Debug,Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uuid: Option<String>,
    pub username: String,
    pub password:String,
    pub phone:Option<String>
}

#[derive(serde::Deserialize,Debug)]
pub struct Login{
    pub username:String,
    pub password:String
}

