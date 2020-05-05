use dirs;
use std::convert::AsRef;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, instrument};
use zip::ZipArchive;

use crate::errors::HostError;

fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
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
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../../../target/workspace.zip"));
    let reader = std::io::Cursor::new(v);
    let mut archive = ZipArchive::new(reader).map_err(|err| HostError::General(err.to_string()))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = expanded_target.join(file.sanitized_name());

        if (&*file.name()).ends_with('/') {
            debug!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}
