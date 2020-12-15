use graphql::{
    utils::{dataset_table_name, dataview_view_name},
    Db,
};
use lambda_http::{
    handler,
    lambda::{self, Context},
    Request, Response,
};
use sqlx::{
    postgres::PgPoolOptions, query, query_scalar, Result as SQLxResult,
};
use std::{collections::HashSet, env};
use tokio_compat_02::FutureExt;
use uuid::Uuid;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    lambda::run(handler(lambda_handler)).compat().await?;
    Ok(())
}

async fn lambda_handler(
    _: Request,
    _: Context,
) -> Result<Response<String>, GenericError> {
    garbage_collect().await?;
    Response::builder()
        .header("Content-Type", "application/json")
        .body("success".into())
        .map_err(|e| e.into())
}

async fn garbage_collect() -> Result<(), GenericError> {
    let docker_db_url = "postgres://postgres@172.17.0.1:5432/motoko".to_owned();
    let db_url = env::var("DATABASE_URL").unwrap_or(docker_db_url.clone());
    let data_db_url =
        env::var("DATA_DATABASE_URL").unwrap_or(docker_db_url + "_data");
    let db = PgPoolOptions::new().connect(&db_url).compat().await?;
    let data_db = PgPoolOptions::new().connect(&data_db_url).compat().await?;
    drop_unreferenced_datasets(&db, &data_db).await?;
    drop_unreferenced_dataviews(&db, &data_db).await?;
    delete_expired_refresh_tokens(&db).await?;
    Ok(())
}

async fn drop_unreferenced_datasets(db: &Db, data_db: &Db) -> SQLxResult<()> {
    let live: HashSet<String> =
        query_scalar::<_, Uuid>("SELECT uuid FROM datasets")
            .fetch_all(db)
            .await?
            .iter()
            .map(dataset_table_name)
            .collect();
    let extant: HashSet<String> = query_scalar::<_, String>(
        r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_type = 'BASE TABLE'
        AND table_schema = 'public'
        "#,
    )
    .fetch_all(data_db)
    .await?
    .into_iter()
    .collect();
    let to_drop: Vec<String> = extant.difference(&live).cloned().collect();
    query(&format!(
        "DROP TABLE IF EXISTS {} CASCADE",
        to_drop.join(",")
    ))
    .execute(data_db)
    .await
    .map(|_| ())
}

async fn drop_unreferenced_dataviews(db: &Db, data_db: &Db) -> SQLxResult<()> {
    let live: HashSet<String> =
        query_scalar::<_, Uuid>("SELECT uuid FROM dataviews")
            .fetch_all(db)
            .await?
            .iter()
            .map(dataview_view_name)
            .collect();
    let extant: HashSet<String> = query_scalar::<_, String>(
        r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_type = 'VIEW'
        AND table_schema = 'public'
        "#,
    )
    .fetch_all(data_db)
    .await?
    .into_iter()
    .collect();
    let to_drop: Vec<String> = extant.difference(&live).cloned().collect();
    query(&format!(
        "DROP VIEW IF EXISTS {} CASCADE",
        to_drop.join(",")
    ))
    .execute(data_db)
    .await
    .map(|_| ())
}

async fn delete_expired_refresh_tokens(db: &Db) -> SQLxResult<()> {
    query("DELETE FROM user_refresh_tokens WHERE expires_at < NOW()")
        .execute(db)
        .await
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use lambda_http::request::from_str;

    #[tokio::test]
    async fn trigger_garbage_collect() -> Result<(), GenericError> {
        dotenv().ok();
        let req = from_str(include_str!(
            "../../tests/data/apigw_v2_proxy_request_get.json"
        ))?;
        let v = lambda_handler(req, Context::default()).compat().await;
        assert!(v.is_ok());
        Ok(())
    }
}
