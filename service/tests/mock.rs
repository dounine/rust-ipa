use crate::prepare::prepare_mock_db;

mod prepare;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();
}
