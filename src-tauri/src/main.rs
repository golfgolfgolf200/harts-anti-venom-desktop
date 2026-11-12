#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Serialize, Deserialize, Debug)]
struct AddressAnalysis {
    address: String,
    is_valid_format: bool,
    prefix: String,
    middle_segment: String,
    suffix: String,
    color_blocks: Vec<String>,
    threat_flag: Option<String>,
}

#[tauri::command]
fn inspect_address(target: String) -> AddressAnalysis {
    let clean = target.trim().to_lowercase();
    let is_valid = clean.starts_with("0x")
        && clean.len() == 42
        && clean[2..].chars().all(|c| c.is_ascii_hexdigit());

    if !is_valid {
        return AddressAnalysis {
            address: clean,
            is_valid_format: false,
            prefix: String::new(),
            middle_segment: String::new(),
            suffix: String::new(),
            color_blocks: vec![],
            threat_flag: Some("Invalid EVM hexadecimal address format".into()),
        };
    }

    let prefix: String = clean.chars().take(6).collect();
    let middle_segment: String = clean.chars().skip(16).take(10).collect();
    let suffix: String = clean.chars().rev().take(6).collect::<String>().chars().rev().collect();

    // Deterministic 5-block Keccak-256 visual hash
    let mut hasher = Keccak256::new();
    hasher.update(clean.as_bytes());
    let hash = hasher.finalize();

    let mut color_blocks = Vec::with_capacity(5);
    for i in 0..5 {
        let r = hash[i * 3];
        let g = hash[i * 3 + 1];
        let b = hash[i * 3 + 2];
        color_blocks.push(format!("#{:02x}{:02x}{:02x}", r, g, b));
    }

    AddressAnalysis {
        address: clean,
        is_valid_format: true,
        prefix,
        middle_segment,
        suffix,
        color_blocks,
        threat_flag: None,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inspect_address])
        .run(tauri::generate_context!())
        .expect("error while running Hart's Anti-Venom desktop application");
}
