pub mod context;

use context::PocketIcTestContext;

#[tokio::test]
async fn test_should_increment_counter() {
    let ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_get_stored_posts() {
    let ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    // wait for 5 seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let posts = ctx.storage_posts().await.expect("posts should be fetched");
    assert_eq!(posts.len(), 1);
}
