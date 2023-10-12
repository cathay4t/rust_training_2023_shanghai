// SPDX-License-Identifier: Apache-2.0

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html_content = reqwest::get("https://devconf.cz").await?.text().await?;
    if let Some((_, title_right_part)) = html_content.split_once("<title>") {
        if let Some((title, _)) = title_right_part.split_once("</title>") {
            println!("Got devconf.cz web title: {title}");
            return Ok(());
        }
    }
    Err("Failed to get devconf.cz web title".into())
}
