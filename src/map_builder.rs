extern crate strfmt;

use reqwest::header::USER_AGENT;
use std::collections::HashMap;
use std::env;
use strfmt::strfmt;
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn _build_tile_url(x: u32, y: u32, z: u8, source: &str) -> String {
    println!("_build_tile_url({}, {}, {}, {})", x, y, z, source);

    let mut coords = HashMap::new();
    coords.insert("x".to_string(), x);
    coords.insert("y".to_string(), y);
    coords.insert("z".to_string(), z.into());
    strfmt(&source.to_string(), &coords)
        .unwrap_or_else(|e| panic!("Failed to build tile URL: {}", e))
}

async fn _get_tile(x: u32, y: u32, z: u8, source: &str) -> Result<()> {
    println!("_get_tile({}, {}, {}, {})", x, y, z, source);

    let url = _build_tile_url(x, y, z, source);
    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header(USER_AGENT, "Mapex framework")
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn get_tiles(x: u32, y: u32, z: u8, source: &str) -> Result<()> {
    println!("get_tiles({}, {}, {}, {})", x, y, z, source);

    _get_tile(x, y, z, source).await
}
