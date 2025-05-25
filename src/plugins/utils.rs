use std::{error::Error, fs, path::Path};

use bzip2::read::BzDecoder;
use tar::Archive;

pub fn extract_bz2(data: &[u8], dest: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dest)?;
    let reader = BzDecoder::new(data);
    let mut archive = Archive::new(reader);
    archive.unpack(dest)?;

    Ok(())
}
