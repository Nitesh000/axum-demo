#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Nitesh").await?.print().await?;

    let req_login = hc.do_post("/api/login",
    json!({
        "usernmae": "nitesh",
        "pwd": "welcome"
    }));
    req_login.await?.print().await?;

    hc.do_get("/hello2/Nitesh").await?.print().await?;

    Ok(())
}
