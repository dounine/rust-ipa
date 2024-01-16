pub fn now() -> chrono::NaiveDateTime {
    chrono::Local::now().naive_local()
}
