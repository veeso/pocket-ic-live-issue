use std::sync::Arc;

use anyhow::Result;
use did::Post;
use ic_canister_client::PocketIcClient;

use candid::{Encode, Principal};
use ic_exports::pocket_ic::PocketIc;

mod wasm;

pub struct PocketIcTestContext {
    pub client: Arc<PocketIc>,
    pub canister: Principal,
}

impl PocketIcTestContext {
    pub async fn new() -> Result<Self> {
        let pocket_ic = ic_exports::pocket_ic::init_pocket_ic()
            .await
            .build_async()
            .await;

        let mut pocket_ic_instance = PocketIc::new_from_existing_instance(
            pocket_ic.get_server_url(),
            pocket_ic.instance_id,
            Some(300_000), // is default
        );

        let mut ctx = PocketIcTestContext {
            client: Arc::new(pocket_ic),
            canister: Principal::anonymous(),
        };

        let principal = ctx
            .create_canister()
            .await
            .expect("canister should be created");
        println!("Created canister with principal {}", principal);

        ctx.canister = principal;

        ctx.install_canister().await?;

        // make live
        pocket_ic_instance
            .set_time(std::time::SystemTime::now())
            .await;
        pocket_ic_instance.make_live(None).await;

        Ok(ctx)
    }

    async fn create_canister(&self) -> Result<Principal> {
        let principal = self
            .client
            .create_canister_with_settings(Some(Self::admin()), None)
            .await;
        self.client.add_cycles(principal, u128::MAX).await;
        Ok(principal)
    }

    async fn install_canister(&self) -> Result<()> {
        let wasm = wasm::get_canister_bytecode().await;
        println!("wasm size: {}", wasm.len());
        self.client
            .install_canister(
                self.canister,
                wasm,
                Encode!(&()).expect("failed to encode"),
                Some(Self::admin()),
            )
            .await;
        Ok(())
    }

    pub fn admin() -> Principal {
        Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap()
    }

    pub async fn get_counter(&self) -> u64 {
        self.client().query("get", ()).await.expect("query failed")
    }

    pub async fn increment_counter(&self) {
        let _response: () = self
            .client()
            .update("increment", ())
            .await
            .expect("query failed");
    }

    pub async fn get_posts(&self) -> anyhow::Result<Vec<Post>> {
        let posts = self.client().update("posts", ()).await?;

        Ok(posts)
    }

    pub async fn storage_posts(&self) -> anyhow::Result<Vec<Post>> {
        let posts = self.client().query("storage_posts", ()).await?;

        Ok(posts)
    }

    fn client(&self) -> PocketIcClient {
        PocketIcClient::from_client(self.client.clone(), self.canister, Self::admin())
    }
}
