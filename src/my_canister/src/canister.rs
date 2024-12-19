use std::{cell::RefCell, rc::Rc};

use candid::Principal;
use did::Post;
use ic_canister::{generate_idl, init, query, update, Canister, Idl, PreUpdate};
use ic_exports::ic_cdk::api::management_canister::http_request::{
    self, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

thread_local! {
    pub static COUNTER: Rc<RefCell<u64>> = Rc::default();
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
                let message = format!(
                    "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                );

                //Return the error as a string and end the method
                ic_exports::ic_cdk::trap(&format!("Call rejected {}", message));
            }
        }
    }

    pub fn idl() -> Idl {
        generate_idl!()
    }
}
