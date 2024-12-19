use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;
use tokio::io::AsyncReadExt as _;

const MY_CANISTER_WASM: &str = "my_canister.wasm.gz";

fn get_workspace_root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub async fn get_canister_bytecode() -> Vec<u8> {
    static CANISTER_BYTECODE: OnceCell<Vec<u8>> = OnceCell::new();
    get_or_load_wasm(&CANISTER_BYTECODE, MY_CANISTER_WASM).await
}

async fn get_or_load_wasm(cell: &OnceCell<Vec<u8>>, file_name: &str) -> Vec<u8> {
    match cell.get() {
        Some(code) => code.clone(),
        None => {
            let code = load_wasm_bytecode_or_panic(file_name).await;
            _ = cell.set(code.clone());
            code
        }
    }
}

async fn load_wasm_bytecode_or_panic(wasm_name: &str) -> Vec<u8> {
    let path = get_path_to_file(wasm_name).await;

    let mut f = tokio::fs::File::open(path)
        .await
        .expect("File does not exists");

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .await
        .expect("Could not read file content");

    buffer
}

pub async fn get_path_to_file(file_name: &str) -> PathBuf {
    if let Ok(dir_path) = std::env::var("WASM_DIR") {
        let file_path = Path::new(&dir_path).join(file_name);
        if check_file_exists(&file_path).await {
            return file_path;
        }
    } else {
        const ARTIFACT_PATH: &str = ".artifact";
        // Get to the root of the project
        let root_dir = get_workspace_root_dir();
        let file_path = root_dir.join(ARTIFACT_PATH).join(file_name);
        if check_file_exists(&file_path).await {
            return file_path;
        }
    }

    if let Ok(dir_path) = std::env::var("DFX_WASMS_DIR") {
        let file_path = Path::new(&dir_path).join(file_name);
        if check_file_exists(&file_path).await {
            return file_path;
        }
    }

    panic!(
        "File {file_name} was not found in dirs provided by ENV variables WASM_DIR or DFX_WASMS_DIR or in the '.artifact' folder"
    );
}

async fn check_file_exists(path: &Path) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}
