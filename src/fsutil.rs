use path_clean::PathClean;
use rand::Rng;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn create_empty_file(p: &Path) -> Result<(), std::io::Error> {
    let f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?;
    f.sync_all()?;
    Ok(())
}

pub fn sync_dir(p: &Path) -> Result<(), std::io::Error> {
    let dir = fs::File::open(p)?;
    dir.sync_all()?;
    Ok(())
}

// Does NOT sync the directory. A sync of the directory still needs to be
// done to ensure the atomic rename is persisted.
pub fn atomic_add_file(p: &Path, contents: &[u8]) -> Result<(), std::io::Error> {
    let temp_path = p
        .to_string_lossy()
        .chars()
        .chain(
            std::iter::repeat(())
                .map(|()| rand::thread_rng().sample(rand::distributions::Alphanumeric))
                .take(8),
        )
        .chain(".tmp".chars())
        .collect::<String>();

    let mut tmp_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)?;
    tmp_file.write_all(contents)?;
    tmp_file.sync_all()?;
    std::fs::rename(temp_path, p)?;
    Ok(())
}

// Get an absolute path without resolving symlinks or touching the fs.
pub fn absolute_path<P>(path: P) -> std::io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}
