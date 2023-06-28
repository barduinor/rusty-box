<img src="images/box-dev-logo.png" 
alt= “box-dev-logo” 
style="margin-left:-10px;"
width=40%;>


# Rusty-Box

[![CI](https://github.com/barduinor/rusty-box/actions/workflows/ci.yml/badge.svg)](https://github.com/barduinor/rusty-box/actions)
[![License](https://img.shields.io/github/license/barduinor/rusty-box)](https://github.com/barduinor/rusty-box/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/rusty-box.svg)](https://crates.io/crates/rusty-box)
[![Docs](https://docs.rs/rusty-box/badge.svg)](https://docs.rs/crate/rusty-box/)

---
Rusty Box is a Rust client for the [Box API](https://developer.box.com/reference/). 

**It is a work in progress and is not yet ready for production use.**

To learn how to use Rusty Box, please refer to the [documentation](https://docs.rs/crate/rusty-box/). There are some [examples that may be useful](./examples) as well.

## Getting Started

### Create a new rust project
    
```bash
cargo new my-box-project
cd my-box-project
``` 
### Add rusty-box to your dependencies

```bash
cargo add dotenv
cargo add rusty-box
```
### Create a .dev.env file in the root of your project

```toml
DEVELOPER_TOKEN=YOUR_DEVELOPER_TOKEN
```

### Open your main.rs file and add the following code

```rust
use rusty_box::{
    auth::{auth_developer::DeveloperToken, AuthError},
    box_client::BoxClient,
    config::Config,
    rest_api::users::users_api,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), AuthError> {
    dotenv::from_filename(".dev.env").ok();

    let config = Config::new();
    let auth = DeveloperToken::new(
        config,
        env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be set"),
    );

    let mut client = BoxClient::new(Box::new(auth.clone()));

    let fields = vec![];

    let me = users_api::me(&mut client, Some(fields)).await?;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
````
### Run your project

```bash
cargo run
```

## Changelog
Please see the [changelog](CHANGELOG.md) for a release history and indications on how to upgrade from one version to another.

## Contributing

If you find any problems or have suggestions about this crate, please submit an issue. Moreover, any pull request, code review and feedback are welcome.


## License

[MIT](./LICENSE)