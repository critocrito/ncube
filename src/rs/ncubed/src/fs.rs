use flate2::read::GzDecoder;
use std::convert::AsRef;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;
use tracing::{debug, instrument};

use crate::errors::HostError;

#[instrument]
pub fn expand_tilde<P: AsRef<Path> + Debug>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

#[instrument]
pub(crate) fn mkdirp<P: AsRef<Path> + Debug>(target: P) -> Result<(), HostError> {
    #[cfg(unix)]
    {
        let expanded_path =
            expand_tilde(target).ok_or(HostError::General("Failed to expand path".into()))?;
        debug!("Creating directory: {:?}", expanded_path);
        fs::create_dir_all(&expanded_path)?;
    }

    #[cfg(windows)]
    {
        debug!("Creating directory: {:?}", target);
        fs::create_dir_all(&target)?;
    }

    Ok(())
}

#[instrument]
pub(crate) fn unzip_workspace<P: AsRef<Path> + Debug>(target: P) -> Result<(), HostError> {
    let expanded_target =
        expand_tilde(target).ok_or(HostError::General("Failed to expand path".into()))?;

    debug!("Extract workspace archive to {:?}", expanded_target);

    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!(
        "../../../../target/workspace/workspace.tar.gz"
    ));
    let tar_gz = std::io::Cursor::new(v);
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(expanded_target)?;

    Ok(())
}
