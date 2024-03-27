use anyhow::{anyhow, Result};
use chrono::Utc;
use std::convert::TryInto;

#[derive(Debug,PartialEq,PartialOrd, Ord, Eq)]
pub struct FileInfo {
    pub mode: u32,
    pub size: u32,
    pub mtime: u32,
    pub mdtime: Option<chrono::DateTime<Utc>>,
    pub path: typed_path::Utf8UnixPathBuf,
}

pub fn parse_file_info<P: AsRef<typed_path::Utf8UnixPath>>(data: Vec<u8>, path: P) -> Result<FileInfo> {
    let mode_bytes = &data[0..4];
    let size_bytes = &data[4..8];
    let mtime_bytes = &data[8..12];

    let mode = u32::from_le_bytes(mode_bytes.try_into()?);
    let size = u32::from_le_bytes(size_bytes.try_into()?);
    let mtime = u32::from_le_bytes(mtime_bytes.try_into()?);
    let mdtime = Some(
        chrono::DateTime::<Utc>::from_timestamp(mtime as i64, 0)
            .ok_or(anyhow!("Parse Datetime Error"))?,
    );

    Ok(FileInfo {
        mode,
        size,
        mtime,
        mdtime,
        path: path.as_ref().to_owned(),
    })
}

#[cfg(feature = "file-extend")]
impl FileInfo {
    pub fn is_dir(&self) -> bool {
        self.mode & 0o170000 == 0o040000
    }

    pub fn is_file(&self) -> bool {
        self.mode & 0o170000 == 0o100000
    }

    pub fn is_symlink(&self) -> bool {
        self.mode & 0o170000 == 0o120000
    }

    pub fn is_fifo(&self) -> bool {
        self.mode & 0o170000 == 0o010000
    }

    pub fn is_char_device(&self) -> bool {
        self.mode & 0o170000 == 0o020000
    }

    pub fn is_block_device(&self) -> bool {
        self.mode & 0o170000 == 0o060000
    }

    pub fn is_socket(&self) -> bool {
        self.mode & 0o170000 == 0o140000
    }

    pub fn is_hidden(&self) -> bool {
        self.path.starts_with('.')
    }
}
