//! Simulating files one step at a time.

use std::fmt;

/// Represents a "file", which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::from(""),
            data: Vec::new(),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File: {}
Data Length: {} bytes",
            self.name,
            self.len()
        )
    }
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn len(&self) -> usize {
      #![doc = r"Returns the file's length in bytes."]
      self.data.len()
    }

    pub fn name(&self) -> String {
      //! Returns the file's name.
      self.name.clone()
    }
}
