use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    hash::Hash,
    path::{Path, PathBuf},
};

/// Represents a path to an asset in the file system.
#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AssetPath<'a> {
    pub path: Cow<'a, Path>,
    pub label: Option<Cow<'a, str>>,
}

impl<'a> Debug for AssetPath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for AssetPath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())?;
        if let Some(label) = &self.label {
            write!(f, "#{label}")?;
        }
        Ok(())
    }
}

impl<'a> AssetPath<'a> {
    /// Creates a new asset path using borrowed information.
    #[inline]
    pub fn new_ref(path: &'a Path, label: Option<&'a str>) -> AssetPath<'a> {
        AssetPath {
            path: Cow::Borrowed(path),
            label: label.map(Cow::Borrowed),
        }
    }

    /// Creates a new asset path.
    #[inline]
    pub fn new(path: PathBuf, label: Option<String>) -> AssetPath<'a> {
        AssetPath {
            path: Cow::Owned(path),
            label: label.map(Cow::Owned),
        }
    }

    /// Gets the sub-asset label.
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|label| label.as_ref())
    }

    /// Gets the path to the asset in the filesystem.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets the path to the asset in the filesystem.
    #[inline]
    pub fn without_label(&self) -> AssetPath<'_> {
        AssetPath::new_ref(&self.path, None)
    }

    /// Gets the path to the asset in the filesystem.
    #[inline]
    pub fn remove_label(&mut self) -> Option<Cow<'a, str>> {
        self.label.take()
    }

    /// Returns this asset path with the given label. This will replace the previous
    /// label if it exists.
    #[inline]
    pub fn with_label(&self, label: impl Into<Cow<'a, str>>) -> AssetPath<'a> {
        AssetPath {
            path: self.path.clone(),
            label: Some(label.into()),
        }
    }

    /// Converts the borrowed path data to owned.
    #[inline]
    pub fn to_owned(&self) -> AssetPath<'static> {
        AssetPath {
            path: Cow::Owned(self.path.to_path_buf()),
            label: self
                .label
                .as_ref()
                .map(|value| Cow::Owned(value.to_string())),
        }
    }

    /// Returns the full extension (including multiple '.' values).
    /// Ex: Returns "config.ron" for "my_asset.config.ron"
    pub fn get_full_extension(&self) -> Option<String> {
        let file_name = self.path.file_name()?.to_str()?;
        let index = file_name.find('.')?;
        let extension = file_name[index + 1..].to_lowercase();
        Some(extension)
    }

    pub(crate) fn iter_secondary_extensions(full_extension: &str) -> impl Iterator<Item = &str> {
        full_extension.chars().enumerate().filter_map(|(i, c)| {
            if c == '.' {
                Some(&full_extension[i + 1..])
            } else {
                None
            }
        })
    }
}

impl<'a> From<&'a str> for AssetPath<'a> {
    fn from(asset_path: &'a str) -> Self {
        let mut parts = asset_path.splitn(2, '#');
        let path = Path::new(parts.next().expect("Path must be set."));
        let label = parts.next();
        AssetPath {
            path: Cow::Borrowed(path),
            label: label.map(Cow::Borrowed),
        }
    }
}

impl<'a> From<&'a String> for AssetPath<'a> {
    fn from(asset_path: &'a String) -> Self {
        asset_path.as_str().into()
    }
}

impl<'a> From<&'a Path> for AssetPath<'a> {
    fn from(path: &'a Path) -> Self {
        AssetPath {
            path: Cow::Borrowed(path),
            label: None,
        }
    }
}

impl<'a> From<PathBuf> for AssetPath<'a> {
    fn from(path: PathBuf) -> Self {
        AssetPath {
            path: Cow::Owned(path),
            label: None,
        }
    }
}
