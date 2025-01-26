use serde_json::Value;
use std::collections::HashMap;
use warp::Filter;

async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    // 组装返回值
    let mut result = HashMap::new();
    result.insert("code", Value::Number(200.into()));
    result.insert("message", Value::String(format!("Hello, {}", name)));
    // 转换成 JSON 格式返回
    Ok(warp::reply::json(&result))
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).and_then(hello);

    // 启动服务
    warp::serve(hello).run(([127, 0, 0, 1], 18085)).await;
}
