use anyhow::Result;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use dotenv::dotenv;
use gql::{
    auth::{extract_bearer_token, user_uuid_from_token},
    context_data::{Auth, ContextData},
    models::user::User,
    mutation::MutationRoot,
    query::QueryRoot,
    state::State,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tide::{http::mime, Body, Error, Request, Response, StatusCode};
use url::Url;

fn main() -> Result<()> {
    async_std::task::block_on(serve())
}

async fn serve() -> Result<()> {
    dotenv().ok();
    let addr = Url::parse(&env::var("ADDRESS")?)?;
    let google_oauth2_client_id = env::var("GOOGLE_OAUTH2_CLIENT_ID")?;
    let jwt_secret = env::var("JWT_KEY")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    let context_data = ContextData {
        user: None,
        pool: pool,
        auth: Auth {
            jwt_secret,
            google_oauth2_client_id,
        },
    };
    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);
    let mut app = tide::with_state(State {
        schema,
        context_data,
    });
    async fn graphql(req: Request<State>) -> tide::Result<Response> {
        let schema = req.state().schema.clone();
        let mut d = req.state().context_data.clone();
        let maybe_token = req
            .header("Authorization")
            .and_then(|headers| Some(headers.last().as_str()))
            .and_then(|header| extract_bearer_token(header));
        let mut req = async_graphql_tide::receive_request(req).await?;
        if let Some(token) = maybe_token {
            let user_uuid = user_uuid_from_token(&d.auth.jwt_secret, &token)
                .map_err(|e| {
                    Error::from_str(
                        StatusCode::Unauthorized,
                        format!("{:?}", e),
                    )
                })?;
            d.user =
                Some(User::get(&d.pool, &user_uuid).await.map_err(|e| {
                    Error::from_str(
                        StatusCode::InternalServerError,
                        format!("{:?}", e),
                    )
                })?);
        }
        req = req.data(d);
        async_graphql_tide::respond(schema.execute(req).await)
    }
    async fn playground(_: Request<State>) -> tide::Result<Response> {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        )));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    }
    app.at("/graphql").post(graphql).get(playground);
    tide::log::start();
    app.listen(addr).await?;
    Ok(())
}
