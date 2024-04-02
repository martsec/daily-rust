use serde_json::json;
use tokio;

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=marti").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );

    req_login.await?.print().await?;

    hc.do_get("/hello2/marti").await?.print().await?;

    let req_create_tickets = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticekt ABC",
        }),
    );

    req_create_tickets.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
