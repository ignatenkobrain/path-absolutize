extern crate slash_formatter;

use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use crate::path_dedot::{ParseDot, ParsePrefix};
use crate::Absolutize;

impl Absolutize for Path {
    #[allow(clippy::let_unit_value)]
    fn absolutize(&self) -> io::Result<PathBuf> {
        if self.is_absolute() {
            self.parse_dot()
        } else {
            let prefix = self.get_path_prefix();

            let _cwd = get_cwd_pathbuf!();
            let cwd = get_cwd!(_cwd);

            if let Some(prefix) = prefix {
                let cwd_prefix = cwd.get_path_prefix().unwrap();

                let cwd = cwd.to_str().unwrap();

                let self_str = self.to_str().unwrap();

                let path = &self_str[prefix.as_os_str().to_str().unwrap().len()..];

                let path = if path.is_empty() {
                    PathBuf::from(slash_formatter::add_end_backslash(
                        prefix.as_os_str().to_str().unwrap(),
                    ))
                } else {
                    PathBuf::from(slash_formatter::concat_with_backslash!(
                        prefix.as_os_str().to_str().unwrap(),
                        &cwd[cwd_prefix.as_os_str().to_str().unwrap().len()..],
                        path
                    ))
                };

                path.parse_dot()
            } else {
                let path = Path::join(cwd, self);

                path.parse_dot()
            }
        }
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            let path_lowercase = path.to_str().unwrap().to_lowercase();

            let virtual_root_lowercase = virtual_root.to_str().unwrap().to_lowercase();

            if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                let path_lowercase = path.to_str().unwrap().to_lowercase();

                let virtual_root_lowercase = virtual_root.to_str().unwrap().to_lowercase();

                if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                    return Err(io::Error::from(ErrorKind::InvalidInput));
                }

                Ok(path)
            } else {
                virtual_root.push(path);

                Ok(virtual_root)
            }
        }
    }
}
