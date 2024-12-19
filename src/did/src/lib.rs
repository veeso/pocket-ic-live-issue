use candid::CandidType;
use serde::Deserialize;

#[derive(Deserialize, CandidType, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    id: String,
    title: String,
    author: String,
}
