use std::{sync::{Arc, Mutex}, collections::HashMap};

use axum::{response::IntoResponse, Json};
use serde_json::json;
use tokio::task::JoinHandle;
use crate::Pool;
#[derive(Debug)]
pub struct AppError{
    pub code:i32,
    pub msg:String
}

impl AppError{
    pub fn err(code:i32,msg:String)->Self{
        AppError{
            code,
            msg
        }
    }
}

impl IntoResponse for AppError{
    fn into_response(self) -> axum::response::Response {
        Json(json!({
            "code":self.code,
            "msg":self.msg
        })).into_response()
    }
}


pub struct Share{
    //数据库异步连接池
    pub pool:Pool,
    //发送短信的handle
    pub tasks:Mutex<HashMap<i32,JoinHandle<()>>>
}

impl Share{
    pub fn new(pool:Pool)->Self{
        let mut tasks:Mutex<HashMap<i32,JoinHandle<()>>>=Mutex::new(HashMap::new());
        Share { pool, tasks }
    }
}