use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use graphql::{
    auth::user_from_authorization_header, Auth, ContextData, Error, Mutation,
    Query,
};
use lambda_http::{
    handler,
    lambda::{self, Context},
    Body, Request, Response,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio_compat_02::FutureExt;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    lambda::run(handler(graphql)).compat().await?;
    Ok(())
}

async fn graphql(
    req: Request,
    _: Context,
) -> Result<Response<String>, GenericError> {
    if req.method().as_str() == "GET" {
        return Response::builder()
            .status(200)
            .header("Content-Type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
            .map_err(|e| e.into());
    }
    let google_oauth2_client_id = env::var("GOOGLE_OAUTH2_CLIENT_ID")?;
    let jwt_secret = env::var("JWT_KEY")?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let auth = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());
    let context_data = ContextData {
        user: user_from_authorization_header(auth, &jwt_secret, &pool).await,
        pool: pool,
        auth: Auth {
            jwt_secret,
            google_oauth2_client_id,
        },
    };
    let schema = Schema::new(Query, Mutation, EmptySubscription);
    let payload = match req.body() {
        Body::Text(payload) => Ok(payload),
        _ => Err(Error::BadRequest),
    }?;
    let gql_req: async_graphql::Request =
        serde_json::from_str(&payload).map_err(|_| Error::BadRequest)?;
    let gql_res = schema.execute(gql_req.data(context_data)).await;
    let body = serde_json::to_string(&gql_res)
        .map_err(|e| -> GenericError { e.into() })?;
    Response::builder()
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Response;
    use dotenv::dotenv;
    use lambda_http::request::from_str;

    #[tokio::test]
    async fn bad_request() -> Result<(), GenericError> {
        dotenv().ok();
        let req = from_str(include_str!(
            "../../tests/data/apigw_v2_proxy_request.json"
        ))?;
        let v = graphql(req, Context::default()).compat().await;
        assert!(v.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn get_playground() -> Result<(), GenericError> {
        dotenv().ok();
        let req = from_str(include_str!(
            "../../tests/data/apigw_v2_proxy_request_get.json"
        ))?;
        let v = graphql(req, Context::default()).compat().await;
        assert!(v.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn me_not_logged_in() -> Result<(), GenericError> {
        dotenv().ok();
        let req = from_str(include_str!(
            "../../tests/data/apigw_v2_proxy_request_me.json"
        ))?;
        let v = graphql(req, Context::default()).compat().await?;
        let res: Response = serde_json::from_str(v.body())?;
        assert!(!res.errors.is_empty());
        Ok(())
    }
}
