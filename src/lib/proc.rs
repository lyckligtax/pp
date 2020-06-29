use std::path::PathBuf;
use regex::Regex;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::lib::MockFS as FS;
    } else {
        use crate::lib::FS;
    }
}

pub fn find_proc_from_inodes(inode: &str) -> Option<isize> {
    find_proc_from_inodes_with_fs(inode, Default::default())
}

pub const PROC_GLOB: &str = "/proc/*/fd/*";

fn find_proc_from_inodes_with_fs(inode: &str, fs: FS) -> Option<isize> {
    let proc_rxp = Regex::new(r"^/proc/(\d+)/").expect("Expected proc rxp");
    let socket = PathBuf::from(format!("socket:[{}]", inode));

    for proc_file in fs.glob_iter(PROC_GLOB) {
        if let Ok(src) = proc_file {
            if let Ok(link) = fs.read_link(&src) {
                if link.eq(&socket) {
                    if let Some(captures) = proc_rxp.captures(src.to_str().expect("Expected Link")) {
                        if let Some(group) = captures.get(1) {
                            return Some(group.as_str().parse().unwrap());
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::*;
    use glob::{GlobResult};
    use mockall::predicate::{eq, never};

    #[test]
    fn test_find_proc_terminates_early_on_find() {
        let mut mock = FS::new();

        mock.expect_glob_iter()
            .returning(|_| Box::new(
                vec![
                    GlobResult::Ok(PathBuf::from("/proc/1/fd/link")),
                    GlobResult::Ok(PathBuf::from("/proc/2/fd/link")),
                    GlobResult::Ok(PathBuf::from("/proc/3/fd/link")),
                ].into_iter()
            ));

        mock.expect_read_link()
            .with(eq(PathBuf::from("/proc/1/fd/link")))
            .times(1)
            .returning(|_|Ok(PathBuf::from("not_socket:[1234]")));

        mock.expect_read_link()
            .with(eq(PathBuf::from("/proc/2/fd/link")))
            .times(1)
            .returning(|_|Ok(PathBuf::from("socket:[1234]")));

        // assert no further links are read
        mock.expect_read_link()
            .times(0);


        let found_proc = find_proc_from_inodes_with_fs("1234", mock);
        assert_eq!(found_proc, Some(2))
    }
}