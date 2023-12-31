use super::super::init::DbPool;
use super::super::models::messages::*;
use sqlx::postgres::PgQueryResult;

pub async fn get_messages_by_threadname(
    pool: &DbPool,
    threadname: &str,
) -> anyhow::Result<Vec<MessageModelSql>> {
    match sqlx::query_as::<_, MessageModelSql>("SELECT * FROM messages WHERE thread_name = $1")
        .bind(threadname)
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err.into()),
    }
}

pub async fn post_message(
    pool: &DbPool,
    message: CreateMessageBody,
) -> anyhow::Result<PgQueryResult> {
    let query = "INSERT INTO messages (id, thread_name, role, content) VALUES ($1, $2, $3, $4)";
    match sqlx::query(query)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(message.thread_name)
        .bind(message.role)
        .bind(message.content)
        .execute(pool.as_ref())
        .await
    {
        Ok(res) => Ok(res),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_message(
    pool: &DbPool,
    params: DeleteMessageParams,
) -> anyhow::Result<PgQueryResult> {
    let query = &format!("DELETE FROM messages WHERE id = $1");
    match sqlx::query(&query)
        .bind(params.id)
        .execute(pool.as_ref())
        .await
    {
        Ok(rows) => Ok(rows),
        Err(err) => Err(err.into()),
    }
}
