use axum::{
    Router,
    extract::{TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{self, Request},
    routing::get,
    response::Response,
    middleware::{self, Next},
};

pub async fn my_middleware1<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // do something with `request`...
    println!("我是中间件1");
    let response = next.run(request).await;

    // do something with `response`...
    println!("中间件1执行完毕");
    response
}

pub async fn my_middleware2<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // do something with `request`...
    println!("我是中间件2");
    println!("auth:{:?}",auth);
    let response = next.run(request).await;

    // do something with `response`...
    println!("中间件2执行完毕");
    response
}

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // do something with `request`...
    println!("auth:{:?}",auth);
    let response = next.run(request).await;

    // do something with `response`...
    println!("中间件2执行完毕");
    response
}