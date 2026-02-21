use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::{borrow::Cow, fs, path::PathBuf};

pub struct FileAssetSource {
    base: PathBuf,
}

impl FileAssetSource {
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }
}

impl AssetSource for FileAssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|bytes| Some(Cow::Owned(bytes)))
            .map_err(Into::into)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(Into::into)
    }
}
