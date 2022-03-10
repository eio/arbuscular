# Step-by-step Setup

If you're using Sublime Text, it's worth installing this package:
https://github.com/rust-lang/rust-enhanced

## Rust setup (macOS)

`brew install rustup`

`rustup-init`

`rustc --version`

`cargo install cargo-edit`

# Running tests

`cargo test`

# Running arbuscular

TODO


---------------------------------------------------------------------------
---------------------------------------------------------------------------
---------------------------------------------------------------------------


# Initial Project Setup (notes for posterity; no need to repeat)

`cargo new arbuscular`

`cd arbuscular`

## Add httpmock example and deps

`mkdir tests`

`cargo add httpmock --dev`

`cargo add isahc`

`cargo add serde`

`cargo add serde_json`

`cargo add serde_with --features "json"`

`cargo add async-std --features "attributes"`

Copy `httpmock` test examples into `/tests/`

### httpmock test examples

https://github.com/alexliesenfeld/httpmock/tree/master/tests/examples


---------------------------------------------------------------------------
---------------------------------------------------------------------------
---------------------------------------------------------------------------

#### initial attempt with wiremock-rs, not used due to lack of github examples

Copy example from https://docs.rs/wiremock/latest/wiremock/ to `/tests`

`cargo add wiremock --dev` // alternative mock lib

`cargo add surf` // alternative HTTP lib to `isahc`