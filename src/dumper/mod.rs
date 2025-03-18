use crate::db::Entry;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub(crate) fn dump_kv_store(
    kv: Vec<(String, Entry)>,
    file_path: Option<&str>,
) -> crate::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&kv).expect("Failed to serialize KV store");

    let path = match file_path {
        Some(p) => p.to_string(),
        None => {
            let now = Local::now();
            format!("{}.bin", now.format("%Y%m%d%H%M%S"))
        }
    };

    let mut file = File::create(path)?;
    println!("wrote file to {:?}", file);
    file.write_all(&encoded)?;
    Ok(())
}
