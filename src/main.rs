mod lib;
use std::env;
use crate::lib::{scan_inodes_for_port, find_proc_from_inodes};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return
    }

    if let Some(inode) = scan_inodes_for_port(&args[1]) {
        if let Some(proc) = find_proc_from_inodes(inode.as_str()) {
            println!("{}", proc)
        }
    }
}
