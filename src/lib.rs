use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

/// A simple Spin HTTP component.
#[http_component]
fn handle_kv_demo(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/azure/:key", set_value);
    router.get("/azure/:key", get_value_by_key);
    router.post("/azure/bulk", bulk_upsert);
    router.get("/azure/keys/all", get_all_keys);
    router.get("/azure/keys/count", get_key_count);
    Ok(router.handle(req))
}

#[derive(Deserialize)]
pub struct SetValueModel {
    pub value: String,
}

#[derive(Deserialize, Serialize)]
pub struct BulkUpsertModel {
    pub values: HashMap<String, String>,
}

fn set_value(req: Request, p: Params) -> Result<impl IntoResponse> {
    // safe to unwrap here because the method is guarded by the router
    let key = p.get("key").unwrap();

    let Ok(model) = serde_json::from_slice::<SetValueModel>(req.body()) else {
        // we received an invalid payload, let's tell the callee..
        return Ok(Response::new(400, "Invalid Payload Received"));
    };

    let store = Store::open("azure")?;
    store.set(key, model.value.as_bytes())?;

    // we got here so everything worked as expected... let's create a response
    Ok(ResponseBuilder::new(201)
        .header("location", format!("/azure/{}", key))
        .body(())
        .build())
}

fn get_value_by_key(_: Request, p: Params) -> Result<impl IntoResponse> {
    // safe to unwrap because the router is guarding this
    let key = p.get("key").unwrap();
    let store = Store::open("azure")?;
    Ok(match store.get(key)? {
        Some(v) => Response::new(200, v),
        None => Response::new(404, ()),
    })
}

fn get_all_keys(_: Request, _: Params) -> Result<impl IntoResponse> {
    let store = Store::open("azure")?;
    let keys = store.get_keys()?;
    let payload = serde_json::to_string_pretty(&keys)?;
    Ok(Response::new(200, payload))
}

fn get_key_count(_: Request, _: Params) -> Result<impl IntoResponse> {
    let store = Store::open("azure")?;
    let keys = store.get_keys()?.len();
    let payload = serde_json::to_string_pretty(&keys)?;
    Ok(Response::new(200, payload))
}

fn bulk_upsert(req: Request, _: Params) -> Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<BulkUpsertModel>(req.body()) else {
        // we received an invalid payload, let's tell the callee..
        return Ok(Response::new(400, "Invalid Payload Received"));
    };
    let store = Store::open("azure")?;
    for key_value in model.values {
        match store.set(key_value.0.as_str(), key_value.1.as_bytes()) {
            Ok(_) => println!("Key {} persisted", key_value.0),
            Err(e) => println!("Failed to persist at key {}: {}", key_value.0, e),
        }
    }
    Ok(Response::new(200, ()))
}
