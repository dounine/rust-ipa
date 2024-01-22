pub fn uuid32() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}
