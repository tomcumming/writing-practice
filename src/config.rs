use std::{borrow::Borrow, sync::LazyLock};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub stroke_order_data: String,
}

static CONFIG: LazyLock<Result<Config, String>> = LazyLock::new(|| {
    toml::from_str(
        std::fs::read_to_string("config.toml")
            .map_err(|e| e.to_string())?
            .as_str(),
    )
    .map_err(|e| e.to_string())
});

pub fn load_config() -> &'static Result<Config, String> {
    CONFIG.borrow()
}
