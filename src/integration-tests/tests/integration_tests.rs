pub mod context;

use context::PocketIcTestContext;

#[tokio::test]
async fn test_should_increment_counter_1() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_2() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_3() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_4() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_5() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_6() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_7() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_increment_counter_8() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let counter = ctx.get_counter().await;
    assert_eq!(counter, 0);

    ctx.increment_counter().await;
    let counter = ctx.get_counter().await;
    assert_eq!(counter, 1);
}

#[tokio::test]
async fn test_should_get_posts_1() {
    let mut ctx = PocketIcTestContext::new()
        .await
        .expect("context should be created");

    ctx.live().await;

    let posts = ctx.get_posts().await.expect("posts should be fetched");
    assert_eq!(posts.len(), 1);
}
