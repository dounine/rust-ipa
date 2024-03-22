pub fn now() -> chrono::NaiveDateTime {
    chrono::Local::now().naive_local()
}
pub fn time_format(time: chrono::NaiveDateTime) -> String {
    //转多少分钟,小时,天,周,月,年前
    let now = now();
    let duration = now.signed_duration_since(time);
    if duration.num_seconds() < 60 {
        return "刚刚".to_string();
    } else if duration.num_minutes() < 60 {
        return format!("{}分钟前", duration.num_minutes());
    } else if duration.num_hours() < 24 {
        return format!("{}小时前", duration.num_hours());
    } else if duration.num_days() < 7 {
        return format!("{}天前", duration.num_days());
    } else if duration.num_weeks() < 4 {
        return format!("{}周前", duration.num_weeks());
    } else if duration.num_days() < 365 {
        return format!("{}个月前", duration.num_days() / 30);
    } else {
        return format!("{}年前", duration.num_days() / 365);
    }
}
