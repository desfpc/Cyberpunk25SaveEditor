use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Default, Deserialize, Serialize)]
pub struct Settings {
    pub last_save_directory: Option<PathBuf>,
}
impl Settings {
    fn path() -> Option<PathBuf> {
        let root = if cfg!(target_os = "windows") {
            std::env::var_os("APPDATA").map(PathBuf::from)
        } else if cfg!(target_os = "macos") {
            std::env::var_os("HOME").map(|p| PathBuf::from(p).join("Library/Application Support"))
        } else {
            std::env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")))
        }?;
        Some(root.join("NightCitySaveEditor/settings.json"))
    }
    pub fn load() -> Self {
        Self::path()
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }
    pub fn remember(&mut self, directory: &Path) -> Result<()> {
        self.last_save_directory = Some(directory.to_owned());
        let path = Self::path().context("Cannot locate settings directory")?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preferences_preserve_unicode_paths_and_do_not_persist_replace_mode() {
        let settings = Settings {
            last_save_directory: Some(PathBuf::from("/tmp/Сохранения/ManualSave-33")),
        };
        let bytes = serde_json::to_vec(&settings).unwrap();
        let restored: Settings = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(restored.last_save_directory, settings.last_save_directory);
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value.as_object().unwrap().len(), 1);
        assert!(Settings::default().last_save_directory.is_none());
    }
}
