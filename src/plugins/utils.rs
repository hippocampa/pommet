use futures_util::StreamExt;
use std::error::Error;
use std::io::{Write, stdout};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download_plugin(plugin_url: &str, save_to: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(plugin_url).await?;
    let total_size = res.content_length().unwrap_or(0);
    let mut file = File::create(save_to).await?;

    let mut downloaded = 0;
    let mut stream = res.bytes_stream();
    let mut last_percentage = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let percentage = (downloaded as f64 / total_size as f64 * 100.0) as u64;

            if percentage > last_percentage {
                print!("\rDownload: {}%", percentage);
                stdout().flush()?;
                last_percentage = percentage;
            }
        } else {
            if downloaded % 102400 == 0 {
                // Update roughly every 100KB
                print!("\rDownload: {} KB", downloaded / 1024);
                stdout().flush()?;
            }
        }
    }

    Ok(())
}
