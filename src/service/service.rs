use sqlx::Executor;

use crate::pool::pool::pool;

pub async fn get_service() -> String {
    pool()
        .await
        .execute(
            "
    INSERT INTO my_table (column1,column2) values ('god of','rust')
    ",
        )
        .await
        .expect("Couldnt execute query");

    return String::from("query complete");
}
