use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();

    hc.do_get("/hello-world?name=tiendang")
        .await?
        .print()
        .await?;

    Ok(())
}
