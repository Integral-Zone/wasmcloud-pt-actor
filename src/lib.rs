use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{IncrementRequest, KeyValue, KeyValueSender};
use wasmcloud_interface_logging::{debug, error, info, warn};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct KvCounterActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for KvCounterActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();

        match (req.method.as_ref(), trimmed_path.as_slice()) {
            ("GET", ["api", "counter"]) => increment_counter(ctx, "default", 1).await,
            ("GET", ["api", "actor"]) => get_actor_response().await,
            ("GET", ["api", "actor_log"]) => get_actor_logger_response().await,
            ("GET", ["api", "counter_log", counter]) => increment_counter_with_log(ctx, counter, 1).await,
            ("GET", ["api", "counter_twice", counter]) => increment_counter_twice(ctx, counter, 1).await,
            ("GET", ["api", "counter_twice_log", counter]) => increment_counter_twice_log(ctx, counter, 1).await,
            ("GET", ["api", "counter", counter]) => increment_counter(ctx, counter, 1).await,
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn get_actor_response() -> RpcResult<HttpResponse> {
    Ok(HttpResponse {
        body: json!({ "counter": 100 }).to_string().as_bytes().to_vec(),
        status_code: 200,
        ..Default::default()
    })
}

async fn get_actor_logger_response() -> RpcResult<HttpResponse> {
    debug!("This is logged at DEBUG level");
    info!("This is logged at INFO level");
    warn!("This is logged at WARN level");
    error!("This is logged at ERROR level");
    Ok(HttpResponse {
        body: json!({ "counter": 200 }).to_string().as_bytes().to_vec(),
        status_code: 200,
        ..Default::default()
    })
}

/// Increment the `key` in the KeyValue store by `value`
async fn increment_counter_with_log(ctx: &Context, counter: &str, value: i32) -> RpcResult<HttpResponse> {
    debug!("This is logged at DEBUG level");
    info!("This is logged at INFO level");
    warn!("This is logged at WARN level");
    error!("This is logged at ERROR level");

    // make friendlier key
    let key = format!("counter:{}", counter.replace('/', ":"));

    // increment the value in kv and format response as json
    let (body, status_code) = match KeyValueSender::new()
        .increment(ctx, &IncrementRequest { key, value })
        .await
    {
        Ok(v) => (json!({ "counter": v }).to_string(), 200),
        // if we caught an error, return it to client
        Err(e) => (json!({ "error": e.to_string() }).to_string(), 500),
    };

    Ok(HttpResponse {
        body: body.as_bytes().to_vec(),
        status_code,
        ..Default::default()
    })
}

async fn increment_counter(ctx: &Context, counter: &str, value: i32) -> RpcResult<HttpResponse> {
    // make friendlier key
    let key = format!("counter:{}", counter.replace('/', ":"));

    // increment the value in kv and format response as json
    let (body, status_code) = match KeyValueSender::new()
        .increment(ctx, &IncrementRequest { key, value })
        .await
    {
        Ok(v) => (json!({ "counter": v }).to_string(), 200),
        // if we caught an error, return it to client
        Err(e) => (json!({ "error": e.to_string() }).to_string(), 500),
    };

    Ok(HttpResponse {
        body: body.as_bytes().to_vec(),
        status_code,
        ..Default::default()
    })
}

async fn increment_counter_twice(ctx: &Context, counter: &str, value: i32) -> RpcResult<HttpResponse> {
    
    // make friendlier key
    let  key = format!("counter:{}", counter.replace('/', ":"));
    let  key2 = format!("counter:{}", counter.replace('/', ":"));

    // let sender = KeyValueSender::new();

    KeyValueSender::new().increment(ctx, &IncrementRequest { key, value }).await?;

    // increment the value in kv and format response as json
    let (body, status_code) = match KeyValueSender::new()
        .increment(ctx, &IncrementRequest { key: key2, value })
        .await
    {
        Ok(v) => (json!({ "counter": v }).to_string(), 200),
        // if we caught an error, return it to client
        Err(e) => (json!({ "error": e.to_string() }).to_string(), 500),
    };

    Ok(HttpResponse {
        body: body.as_bytes().to_vec(),
        status_code,
        ..Default::default()
    })
}

async fn increment_counter_twice_log(ctx: &Context, counter: &str, value: i32) -> RpcResult<HttpResponse> {
    
    debug!("This is logged at DEBUG level");
    info!("This is logged at INFO level");
    warn!("This is logged at WARN level");
    error!("This is logged at ERROR level");

    // make friendlier key
    let  key = format!("counter:{}", counter.replace('/', ":"));
    let  key2 = format!("counter:{}", counter.replace('/', ":"));

    // let sender = KeyValueSender::new();

    KeyValueSender::new().increment(ctx, &IncrementRequest { key, value }).await?;

    // increment the value in kv and format response as json
    let (body, status_code) = match KeyValueSender::new()
        .increment(ctx, &IncrementRequest { key: key2, value })
        .await
    {
        Ok(v) => (json!({ "counter": v }).to_string(), 200),
        // if we caught an error, return it to client
        Err(e) => (json!({ "error": e.to_string() }).to_string(), 500),
    };

    Ok(HttpResponse {
        body: body.as_bytes().to_vec(),
        status_code,
        ..Default::default()
    })
}