use std::path::Path;
use regex::bytes::Regex;
use std::io::{BufRead, BufReader};

const PROTOCOLS: [&str; 4] = ["tcp", "tcp6", "udp", "udp6"];

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::lib::MockFS as FS;
    } else {
        use crate::lib::FS;
    }
}

pub fn scan_inodes_for_port(port: &str) -> Option<String> {
    scan_inodes_for_port_with_fs(port, Default::default())
}

pub fn scan_inodes_for_port_with_fs(port: &str, fs: FS) -> Option<String> {
    let proc_file_dir: &Path = Path::new("/proc/net/");

    let inode_rxp = Regex::new(r"^\s+?(?:\S+\s+){9}(\d+)").expect("Expected valid inode Regexp");
    let port_rxp = Regex::new(&format!(r"^(?:.+?:){{2}}{:04X}(?:.+?:\S+?\s)0A", port)).expect("Expected valid port Regexp");

    for protocol in &PROTOCOLS {
        let proc_file_path = proc_file_dir.join(protocol);
        let proc_file = fs.open(&proc_file_path).expect("Expected a proc file");
        let mut reader = BufReader::new(proc_file);

        let mut buffer: Vec<u8> = vec![];
        while let Ok(r) = reader.read_until(0x0A as u8, &mut buffer) {
            if r == 0 {
                break;
            }
            if !port_rxp.is_match(&buffer) {
                buffer.clear();
                continue;
            }
            let inode = inode_rxp
                .captures(&buffer)
                .expect("Line should have matched inode Regexp")
                .get(1)
                .expect("Expected matched inode");


            let res = String::from_utf8(inode.as_bytes().to_vec()).expect("Expected inode string");
            buffer.clear();
            return Option::Some(res);
        }
    }

    Option::None
}