use futures_util::StreamExt;
use std::error::Error;
use std::fs::{File as StdFile, create_dir_all};
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
use tokio::{fs::File, io::AsyncWriteExt};
use zip::ZipArchive;

pub fn update_progress(
    current: u64,
    total: u64,
    last_percentage: &mut u64,
    operation: &str,
) -> Result<(), Box<dyn Error>> {
    if total > 0 {
        let percentage = (current as f64 / total as f64 * 100.0) as u64;

        if percentage > *last_percentage {
            print!("\r{}: {}%", operation, percentage);
            stdout().flush()?;
            *last_percentage = percentage;
        }
    } else {
        if current % 102400 == 0 {
            // Update roughly every 100KB
            print!("\r{}: {} KB", operation, current / 1024);
            stdout().flush()?;
        }
    }

    Ok(())
}

pub async fn download_plugin(plugin_url: &str, save_to: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(plugin_url).send().await?;

    // Check status code before proceeding
    if !response.status().is_success() {
        return Err(format!("Failed to download PHP: HTTP status {}", response.status()).into());
    }

    // Get content type to verify we're getting a zip file
    if let Some(content_type) = response.headers().get("content-type") {
        let content_type = content_type.to_str().unwrap_or("unknown");
        if !content_type.contains("application/zip")
            && !content_type.contains("application/octet-stream")
        {
            println!(
                "Warning: Expected zip file but got content-type: {}",
                content_type
            );
        }
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut file = File::create(save_to).await?;

    let mut downloaded = 0;
    let mut stream = response.bytes_stream();
    let mut last_percentage = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        update_progress(downloaded, total_size, &mut last_percentage, "Download")?;
    }

    // Verify the file is not empty
    let file_size = tokio::fs::metadata(save_to).await?.len();
    if file_size < 1000 {
        // Arbitrary small size that's definitely not a valid PHP zip
        return Err("Downloaded file is too small to be a valid PHP archive".into());
    }

    println!("\nDownload complete!");
    Ok(())
}

pub fn unzip(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let dest_path = Path::new(dest);
    if !dest_path.exists() {
        create_dir_all(dest_path)?;
    }

    let file = StdFile::open(src)?;
    let mut archive = ZipArchive::new(file)?;

    let total_files = archive.len() as u64;
    let mut processed_files = 0;
    let mut last_percentage = 0;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(dest).join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p)?;
                }
            }

            let mut outfile = StdFile::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        processed_files += 1;
        update_progress(
            processed_files,
            total_files,
            &mut last_percentage,
            "Extract",
        )?;
    }
    println!("\nExtraction complete!");

    println!("Cleaning up temporary files...");
    match std::fs::remove_file(src) {
        Ok(_) => println!("Removed temporary file: {}", src),
        Err(e) => println!("Warning: Could not remove temporary file {}: {}", src, e),
    }

    Ok(())
}

pub fn write_conf(config_bytes: &[u8], dest: PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = StdFile::create(&dest)?;
    file.write_all(config_bytes)?;
    Ok(())
}
