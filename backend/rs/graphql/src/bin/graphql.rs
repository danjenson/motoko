use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request as GQLRequest,
};
use graphql::{
    auth::user_from_authorization_header, respond, ContextData, Error,
    GenericError,
};
use lambda_http::{
    handler,
    lambda::{self, Context as LambdaContext},
    Body, Request as LambdaRequest, Response,
};
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    lambda::run(handler(lambda_handler)).compat().await?;
    Ok(())
}

async fn lambda_handler(
    req: LambdaRequest,
    _: LambdaContext,
) -> Result<Response<String>, GenericError> {
    if req.method().as_str() == "GET" {
        return Response::builder()
            .status(200)
            .header("Content-Type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
            .map_err(|e| e.into());
    }
    let mut ctx = ContextData::default().await?;
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());
    ctx.user = user_from_authorization_header(
        auth_header,
        &ctx.auth.jwt_secret,
        &ctx.db,
    )
    .await;
    let payload = match req.body() {
        Body::Text(payload) => Ok(payload),
        _ => Err(Error::BadRequest),
    }?;
    let gql_req: GQLRequest =
        serde_json::from_str(&payload).map_err(|_| Error::BadRequest)?;
    let gql_res = respond(gql_req, &ctx).await;
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
        let v = lambda_handler(req, LambdaContext::default()).compat().await;
        assert!(v.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn get_playground() -> Result<(), GenericError> {
        dotenv().ok();
        let req = from_str(include_str!(
            "../../tests/data/apigw_v2_proxy_request_get.json"
        ))?;
        let v = lambda_handler(req, LambdaContext::default()).compat().await;
        assert!(v.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn me_not_logged_in() -> Result<(), GenericError> {
        dotenv().ok();
        let req = from_str(include_str!(
            "../../tests/data/apigw_v2_proxy_request_me.json"
        ))?;
        let v = lambda_handler(req, LambdaContext::default())
            .compat()
            .await?;
        let res: Response = serde_json::from_str(v.body())?;
        assert!(!res.errors.is_empty());
        Ok(())
    }
}
