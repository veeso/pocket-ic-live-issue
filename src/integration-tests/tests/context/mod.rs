use std::time::SystemTime;

use anyhow::Result;
use did::Post;
use serde::de::DeserializeOwned;

use candid::{utils::ArgumentEncoder, CandidType, Decode, Encode, Principal};
use ic_exports::pocket_ic::{PocketIc, PocketIcBuilder, WasmResult};

mod wasm;

pub struct PocketIcTestContext {
    pub client: PocketIc,
    pub canister: Principal,
}

impl PocketIcTestContext {
    pub async fn new() -> Result<Self> {
        Self::new_with(|builder| builder.with_nns_subnet().with_application_subnet()).await
    }

    /// Creates a new test context with the given canisters and custom build and pocket_ic.
    ///
    /// # Arguments
    ///
    /// * `canisters_set` - The set of canisters to create.
    /// * `with_build` - A closure that takes a `PocketIcBuilder` and returns a `PocketIcBuilder`.
    /// * `with_pocket_ic` - A closure that takes a `PocketIc` and returns a `Future` that resolves to a `PocketIc`.
    ///
    /// # Example
    ///
    /// ```
    /// use ic_test_utilities::pocket_ic::PocketIcTestContext;
    /// use ic_test_utilities::context::CanisterType;
    ///
    /// let canisters_set = vec![CanisterType::ICRC1];
    ///
    /// let ctx = PocketIcTestContext::new_with(
    ///    &canisters_set,
    ///    |builder| builder.with_ii_subnet().with_bitcoin_subnet(),
    /// ).await;
    /// ```
    pub async fn new_with<FB>(with_build: FB) -> Result<Self>
    where
        FB: FnOnce(PocketIcBuilder) -> PocketIcBuilder,
    {
        let pocket_ic = ic_exports::pocket_ic::init_pocket_ic_with(with_build)
            .await
            .build_async()
            .await;

        let mut ctx = PocketIcTestContext {
            client: pocket_ic,
            canister: Principal::anonymous(),
        };

        let principal = ctx
            .create_canister()
            .await
            .expect("canister should be created");
        println!("Created canister with principal {}", principal);

        ctx.canister = principal;

        ctx.install_canister().await?;

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

    pub async fn live(&mut self) {
        self.client.make_live(None).await;
        self.client.set_time(SystemTime::now()).await;

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    pub async fn deterministic(&mut self) {
        self.client.stop_live().await;
    }

    pub fn admin() -> Principal {
        Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap()
    }

    pub async fn get_counter(&self) -> u64 {
        self.query("get", ()).await.expect("query failed")
    }

    pub async fn increment_counter(&self) {
        let _response: () = self.update("increment", ()).await.expect("query failed");
    }

    pub async fn get_posts(&self) -> anyhow::Result<Vec<Post>> {
        self.update("posts", ()).await
    }

    /// Performs update call with the given arguments.
    async fn update<T, R>(&self, method: &str, args: T) -> anyhow::Result<R>
    where
        T: ArgumentEncoder + Send + Sync,
        R: DeserializeOwned + CandidType,
    {
        let args = candid::encode_args(args)?;

        let res = self
            .client
            .update_call(self.canister, Self::admin(), method, args)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let reply = match res {
            WasmResult::Reply(reply) => reply,
            WasmResult::Reject(e) => anyhow::bail!("reject error: {:?}", e),
        };

        let decoded = Decode!(&reply, R)?;
        Ok(decoded)
    }

    /// Performs query call with the given arguments.
    async fn query<T, R>(&self, method: &str, args: T) -> anyhow::Result<R>
    where
        T: ArgumentEncoder + Send + Sync,
        R: DeserializeOwned + CandidType,
    {
        let args = candid::encode_args(args)?;

        let call_result = self
            .client
            .query_call(self.canister, Self::admin(), method, args)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let reply = match call_result {
            WasmResult::Reply(reply) => reply,
            WasmResult::Reject(e) => anyhow::bail!("reject error: {:?}", e),
        };

        let decoded = Decode!(&reply, R)?;
        Ok(decoded)
    }
}
