pub fn byte_format(size: i64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = size as f64;
    let mut i = 0;
    while size >= 1024.0 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.2} {}", size, units[i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_format() {
        assert_eq!(byte_format(0), "0.00B");
        assert_eq!(byte_format(1024), "1.00KB");
        assert_eq!(byte_format(1024 * 1024), "1.00MB");
        assert_eq!(byte_format(1024 * 1024 * 1024), "1.00GB");
        assert_eq!(byte_format(1024 * 1024 * 1024 * 1024), "1.00TB");
        assert_eq!(byte_format(1024 * 1024 * 1024 * 1024 * 1024), "1.00PB");
        assert_eq!(byte_format(1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1.00EB");
    }
}
