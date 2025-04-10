use std::{fs, path::PathBuf};
use anyhow::{Ok, Result};

const DEFAULT_MEDIA_STORAGE: &str = "media";

pub(crate) fn get_local_song_file(hex_id: &str) -> Result<fs::File> {
    let (p, f) = hex_id.split_at(2);
    let mut location = PathBuf::from(DEFAULT_MEDIA_STORAGE);

    location.extend([p, f]);
    Ok(fs::File::open(location)?)
}

pub(crate) fn get_all_song_ids() -> Result<Vec<String>> {
    let location = PathBuf::from(DEFAULT_MEDIA_STORAGE);

    Ok(
        fs::read_dir(location)?
            .flat_map(|prefix| fs::read_dir(prefix.unwrap().path()).unwrap())
            .map(|path| 
                "0x".to_string() + &path.unwrap().path().to_str().unwrap()
                .split("/")
                .collect::<Vec<_>>()[1..]
                .join("")
            ).collect()
    )
}

pub(crate) fn store_song(file: &PathBuf, hex_id: &str) -> Result<PathBuf> {
    let (p, f) = hex_id.split_at(2);
    let mut location = PathBuf::from(DEFAULT_MEDIA_STORAGE);

    location.extend([p]);
    if !location.is_dir() {
        fs::create_dir(&location)?;
    }

    location.extend([f]);
    fs::copy(&file, &location)?;

    Ok(location)
}
