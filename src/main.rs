#![windows_subsystem = "windows"]

use desktop_online::post_it;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    post_it().await?;
    Ok(())
}
