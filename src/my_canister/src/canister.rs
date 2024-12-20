use std::{cell::RefCell, rc::Rc, time::Duration};

use candid::Principal;
use did::Post;
use ic_canister::{generate_idl, init, query, update, Canister, Idl, PreUpdate};
use ic_exports::ic_cdk::api::management_canister::http_request::{
    self, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

thread_local! {
    pub static COUNTER: Rc<RefCell<u64>> = Rc::default();

    pub static POSTS: Rc<RefCell<Vec<Post>>> = Rc::default();

}

const CYCLES_PER_HTTP_REQUEST: u128 = 5_000_000_000;

#[derive(Canister, Clone, Debug)]
pub struct MyCanister {
    #[id]
    id: Principal,
}

impl PreUpdate for MyCanister {}

impl MyCanister {
    #[init]
    pub fn init(&mut self) {
        COUNTER.with(|counter| {
            *counter.borrow_mut() = 0;
        });

        async fn fetch_posts() {
            let posts = fetch_posts_from_server().await;
            POSTS.with(|posts_ref| {
                *posts_ref.borrow_mut() = posts;
            });
        }

        ic_exports::ic_cdk_timers::set_timer_interval(Duration::from_millis(200), || {
            ic_exports::ic_cdk::spawn(fetch_posts());
        });
    }

    #[update]
    pub fn increment(&mut self) {
        COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
        });
    }

    #[query]
    pub fn get(&self) -> u64 {
        let mut result = 0;
        COUNTER.with(|counter| {
            result = *counter.borrow();
        });
        result
    }

    #[update]
    pub async fn posts(&self) -> Vec<Post> {
        fetch_posts_from_server().await
    }

    #[query]
    pub async fn storage_posts(&self) -> Vec<Post> {
        POSTS.with(|posts_ref| {
            let posts = posts_ref.borrow();
            posts.clone()
        })
    }

    pub fn idl() -> Idl {
        generate_idl!()
    }
}

async fn fetch_posts_from_server() -> Vec<Post> {
    // get post from <https://jsonplaceholder.typicode.com/posts>
    let request = CanisterHttpRequestArgument {
        url: "http://localhost:3000".to_string(),
        method: HttpMethod::GET,
        body: Some(
            serde_json::json!([
                {
                    "id": "1",
                    "title": "foo",
                    "author": "bar"
                }
            ])
            .to_string()
            .into_bytes(),
        ),
        max_response_bytes: Some(10_000),
        headers: vec![
            HttpHeader {
                name: "Accept".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ],
        transform: None,
    };

    match http_request::http_request(request, CYCLES_PER_HTTP_REQUEST).await {
        Ok((response,)) => {
            let body_as_string =
                std::str::from_utf8(&response.body).expect("failed to parse response");
            println!("response: {}", body_as_string);

            serde_json::from_slice(&response.body).expect("failed to parse response")
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            ic_exports::ic_cdk::trap(&format!("Call rejected {}", message));
        }
    }
}
