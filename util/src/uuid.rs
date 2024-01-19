pub fn uuid32() -> String {
    uuid::Uuid::new_v4().to_simple().to_string()
}
