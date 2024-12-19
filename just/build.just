
# Cleans the build artifacts
[group('build')]
[confirm("Are you sure you want to clean the build artifacts?")]
clean:
  rm -rf {{WASM_DIR}}
  cargo clean

# Builds all canisters
[group('build')]
build_all_canisters: build_my_canister


# Builds the icrc2 bridge canister
[group('build')]
build_my_canister: pre_build
  just build_canister "my_canister" "export-api" "my_canister"

[private]
pre_build: 
  mkdir -p "{{WASM_DIR}}"


# Builds a canister with the given name and features, generates did file then shrinks and gzips the wasm file.
# The output wasm file is saved in the WASM_DIR directory.
[private]
build_canister canister_name features output_wasm:
  echo "Building {{canister_name}} Canister with features: {{features}}"
  cargo build --target wasm32-unknown-unknown --release --package "{{canister_name}}" --features "{{features}}"
  ic-wasm "target/wasm32-unknown-unknown/release/{{canister_name}}.wasm" -o "{{WASM_DIR}}/{{output_wasm}}.wasm" shrink
  candid-extractor "{{WASM_DIR}}/{{output_wasm}}.wasm" > "{{WASM_DIR}}/{{output_wasm}}.did"
  gzip -k "{{WASM_DIR}}/{{output_wasm}}.wasm" --force
