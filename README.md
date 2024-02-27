# pact-zed

Pact Programming Language extension for [Zed](https://github.com/zed-industries/zed)

## Test locally

- Clone this repo: `git clone https://github.com/kadena-community/pact-zed`
- Clone the tree-sitter-pact repo: `git clone https://github.com/kadena-community/tree-sitter-pact`
- CD into the repo: `cd tree-sitter-pact`
- Build the WASM: `tree-sitter build-wasm` (might require docker-engine running)
- Rename the WASM file to `pact.wasm`
- Move the WASM file into `grammars` folder in this repository
- Move the this repository to `~/Library/Application\ Support/Zed/extensions/installed`
- After moving make sure to remove `grammars/pact.toml` and only the `pact.wasm` file exist in the `grammars` folder otherwise zed won't load it correctly
