use sea_orm::*;
use ::entity::user;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [user::Model {
                id: 1,
                nick_name: Some("lake".to_owned()),
                user_name: Some("lake".to_owned()),
                email: None,
                password: None,
                channel_code: "ipa.com".to_owned(),
                ip: "127.0.0.1".to_owned(),
                uid: None,
                avatar: None,
                status: user::UserStatus::Normal.to_owned(),
                platform: user::UserPlatform::Email.to_owned(),
                user_type: user::UserType::User.to_owned(),
                ..Default::default()
            }, ],
        ])
        .exec_results(vec![
            MockExecResult {
                last_insert_id: 2,
                rows_affected: 1,
            },
        ])
        .into_connection()
}