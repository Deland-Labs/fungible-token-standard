pub fn get_image_type(logo: &[u8]) -> Result<String, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_get_logo_type() {
        let mut logo_bytes = Vec::new();
        std::fs::File::open("assets/test.png")
            .unwrap()
            .read_to_end(&mut logo_bytes)
            .unwrap();

        // get logo type
        let logo_type = get_image_type(&logo_bytes).unwrap();
        assert_eq!(logo_type, "image/png");

        let invalid_logo = vec![0x00, 0x01, 0x02, 0x03, 0x04];
        let logo_type_res = get_image_type(&invalid_logo);
        assert!(logo_type_res.is_err());
    }
}
