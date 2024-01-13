pub fn md5<T: AsRef<[u8]>>(data: T) -> String {
    let hash = md5::compute(data.as_ref());
    format!("{:?}", hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let s = "123456";
        assert_eq!(md5(s), "e10adc3949ba59abbe56e057f20f883e");
    }
}
