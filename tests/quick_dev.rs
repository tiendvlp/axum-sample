use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();

    hc.do_get("/hello-world?name=tiendang")
        .await?
        .print()
        .await?;

    hc.do_post(
        "/login",
        json!({
            "userName": "tiendang",
            "userPassword": "password"
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
