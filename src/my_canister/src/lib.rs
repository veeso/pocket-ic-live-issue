pub mod canister;

pub use crate::canister::MyCanister;

#[cfg(target_family = "wasm")]
#[ic_canister::export_candid]
pub fn idl() -> String {
    let canister_idl = MyCanister::idl();

    candid::pretty::candid::compile(&canister_idl.env.env, &Some(canister_idl.actor))
}
