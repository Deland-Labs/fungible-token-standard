pub mod ic_logger;
pub mod principal;
pub mod range_utils;
pub mod sha256;

// fn get logo type
pub fn get_logo_type(logo: &[u8]) -> Result<String, String> {
    let mut logo_type = "".to_string();
    let magic_bytes: [(&[u8], &str); 5] = [
        (b"\x89PNG\r\n\x1a\n", "image/png"),
        (&[0xff, 0xd8, 0xff], "image/jpeg"),
        (b"GIF89a", "image/gif"),
        (b"GIF87a", "image/gif"),
        (b"RIFF", "image/webp"),
    ];

    for &(k, v) in magic_bytes.iter() {
        if logo.len() > k.len() && &logo[0..k.len()] == k {
            logo_type = v.to_string();
            break;
        }
    }
    if logo_type.is_empty() {
        //convert logo bytes to string
        let logo_str = String::from_utf8(logo.to_vec()).unwrap();
        // if logo_str is svg
        if logo_str.contains("<svg") && logo_str.contains("</svg>") {
            logo_type = "image/svg+xml".to_string();
        }
    }
    if logo_type.is_empty() {
        return Err("Unsupported logo type".to_string());
    }
    Ok(logo_type)
}
