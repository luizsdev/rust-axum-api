use sqlx::{query, Executor};

use crate::{handler::handler::BodyParams, pool::pool::pool};

pub async fn get_service(body: BodyParams) -> String {
    pool()
        .await
        .execute(query!(
            "INSERT INTO my_table (column1,column2) VALUES ($1,$2)",
            body.name,
            body.email,
        ))
        .await
        .expect("Couldnt execute query");

    String::from("query complete")
}
