use tokio;

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=marti").await?.print().await?;
    hc.do_get("/hello2/marti").await?.print().await?;

    Ok(())
}
