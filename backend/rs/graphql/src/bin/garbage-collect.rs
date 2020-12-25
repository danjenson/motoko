use graphql::{
    utils::{dataset_table_name, dataview_view_name, run_mode},
    Db, Secrets,
};
use lambda_http::{
    handler,
    lambda::{self, Context},
    Request, Response,
};
use regex::Regex;
use rusoto_core::Region;
use rusoto_s3::{
    Delete, DeleteObjectsRequest, ListObjectsV2Request, ObjectIdentifier,
    S3Client, S3,
};
use sqlx::{
    postgres::PgPoolOptions, query, query_scalar, Result as SQLxResult,
};
use std::collections::HashSet;
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
    let secrets = match run_mode().as_str() {
        "local" => Secrets::docker(),
        _ => Secrets::aws().await?,
    };
    garbage_collect(&secrets).await?;
    Response::builder()
        .header("Content-Type", "application/json")
        .body("success".into())
        .map_err(|e| e.into())
}

async fn garbage_collect(secrets: &Secrets) -> Result<(), GenericError> {
    let db = Db {
        meta: PgPoolOptions::new()
            .max_connections(1)
            .connect(&secrets.meta_db_url)
            .compat()
            .await?,
        data: PgPoolOptions::new()
            .max_connections(1)
            .connect(&secrets.data_db_url)
            .compat()
            .await?,
    };
    let s3 = S3Client::new(Region::UsWest1);
    let bucket = "motoko-data";
    drop_unreferenced_datasets(&db).await?;
    drop_unreferenced_dataviews(&db).await?;
    delete_expired_refresh_tokens(&db).await?;
    delete_unreferenced_objects(&db, &s3, bucket, "plots").await?;
    Ok(())
}

async fn drop_unreferenced_datasets(db: &Db) -> SQLxResult<()> {
    let live: HashSet<String> =
        query_scalar::<_, Uuid>("SELECT uuid FROM datasets")
            .fetch_all(&db.meta)
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
    .fetch_all(&db.data)
    .await?
    .into_iter()
    .collect();
    let to_drop: Vec<String> = extant.difference(&live).cloned().collect();
    query(&format!(
        "DROP TABLE IF EXISTS {} CASCADE",
        to_drop.join(",")
    ))
    .execute(&db.data)
    .await
    .map(|_| ())
}

async fn drop_unreferenced_dataviews(db: &Db) -> SQLxResult<()> {
    let live: HashSet<String> =
        query_scalar::<_, Uuid>("SELECT uuid FROM dataviews")
            .fetch_all(&db.meta)
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
    .fetch_all(&db.data)
    .await?
    .into_iter()
    .collect();
    let to_drop: Vec<String> = extant.difference(&live).cloned().collect();
    query(&format!(
        "DROP VIEW IF EXISTS {} CASCADE",
        to_drop.join(",")
    ))
    .execute(&db.data)
    .await
    .map(|_| ())
}

async fn delete_expired_refresh_tokens(db: &Db) -> SQLxResult<()> {
    query("DELETE FROM user_refresh_tokens WHERE expires_at < NOW()")
        .execute(&db.meta)
        .await
        .map(|_| ())
}

async fn delete_unreferenced_objects(
    db: &Db,
    s3: &S3Client,
    bucket: &str,
    table_name: &str,
) -> Result<(), GenericError> {
    let re = Regex::new(&format!(r"{}/([a-fA-F0-9-]+).*", table_name)).unwrap();
    let live: HashSet<Uuid> =
        query_scalar::<_, Uuid>(&format!("SELECT uuid FROM {}", table_name))
            .fetch_all(&db.meta)
            .await?
            .into_iter()
            .collect();
    let to_delete: Vec<ObjectIdentifier> = s3
        .list_objects_v2(ListObjectsV2Request {
            bucket: bucket.to_owned(),
            prefix: Some(table_name.to_owned()),
            ..Default::default()
        })
        .await?
        .contents
        .unwrap_or(Vec::new())
        .iter()
        .filter(|o| {
            if o.key.is_none() {
                return false;
            }
            let key = o.key.clone().unwrap();
            let m = re.captures(&key);
            if m.is_none() {
                return false;
            }
            !live.contains(
                &Uuid::parse_str(m.unwrap().get(1).unwrap().as_str()).unwrap(),
            )
        })
        .map(|o| ObjectIdentifier {
            key: o.key.clone().unwrap(),
            version_id: None,
        })
        .collect();
    if !to_delete.is_empty() {
        let req = DeleteObjectsRequest {
            bucket: bucket.to_owned(),
            delete: Delete {
                objects: to_delete,
                quiet: None,
            },
            ..Default::default()
        };
        s3.delete_objects(req).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_compat_02::FutureExt;

    #[tokio::test]
    async fn test_garbage_collect() -> Result<(), GenericError> {
        let secrets = Secrets::aws().compat().await?;
        garbage_collect(&secrets).compat().await
    }
}
