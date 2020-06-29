use std::fs::{File, read_link};
use std::path::PathBuf;
use glob::{GlobResult};
use std::io;
#[cfg(test)]
use mockall::automock;

#[derive(Default, Copy, Clone)]
pub struct FS;
#[cfg_attr(test,automock)]
impl FS {
    pub fn read_link(self, path: &PathBuf) -> io::Result<PathBuf> {
        read_link(path)
    }

    pub fn open(self, path: &PathBuf) -> io::Result<File> {
        File::open(path)
    }

    pub fn glob_iter(self, pattern: &str) -> impl Iterator<Item = GlobResult> {
        glob::glob(pattern).expect("Expected correct GLOB pattern")
    }
}