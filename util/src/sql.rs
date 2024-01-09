#[derive(Debug, Clone)]
pub struct Sql(String);

impl From<&str> for Sql {
    fn from(value: &str) -> Self {
        Sql(value.to_string())
    }
}

impl From<String> for Sql {
    fn from(value: String) -> Self {
        Sql(value)
    }
}

impl AsRef<String> for Sql {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Into<String> for Sql {
    fn into(self) -> String {
        self.0
    }
}

unsafe impl Send for Sql {}

unsafe impl Sync for Sql {}

pub trait SqlTrait {
    /// sql字符串压缩
    /// 去除换行符、回车符、多余空格
    /// 用于sql语句的打印
    fn compress(&self) -> Self;
}

impl SqlTrait for Sql {
    fn compress(&self) -> Self {
        self.0
            .replace("\n", "")
            .replace("\r", "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ").into()
    }
}