use crate::{context_data::ContextData, models::user::User};
use async_graphql::{Context, FieldResult, ID};
use std::str;
use uuid::Uuid;

pub fn get_data<'a>(ctx: &'a Context<'_>) -> FieldResult<&'a ContextData> {
    ctx.data::<ContextData>()
}

pub fn current_user<'a>(context: &'a Context<'_>) -> FieldResult<&'a User> {
    let d = context.data::<ContextData>()?;
    match &d.user {
        Some(user) => Ok(user),
        None => Err("not logged in!".into()),
    }
}

pub fn is_current_user(user: &User, context: &Context<'_>) -> FieldResult<()> {
    let curr_user = current_user(context)?;
    if curr_user.uuid != user.uuid {
        Err("invalid permissions".into())
    } else {
        Ok(())
    }
}

pub fn user_name_from_email(email: &str) -> String {
    email
        .split("@")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .replace(".", "_")
}

pub struct DbKeys {
    pub model_name: String,
    pub keys: Vec<Uuid>,
}

pub fn graphql_id_to_db_keys(id: ID) -> DbKeys {
    let model_pks = str::from_utf8(&base64::decode(id.to_string()).unwrap())
        .unwrap()
        .to_string()
        .split(":")
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let (model_name, pk_strs) = model_pks.split_first().unwrap();
    let keys = pk_strs
        .iter()
        .map(|pk_str| Uuid::parse_str(pk_str).unwrap())
        .collect::<Vec<Uuid>>();
    DbKeys {
        model_name: model_name.to_owned(),
        keys,
    }
}
