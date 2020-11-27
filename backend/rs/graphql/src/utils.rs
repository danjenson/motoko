use crate::{models::User, ContextData, Error, ModelKeys};
use async_graphql::{Context, Result, ID};
use std::str;
use uuid::Uuid;

pub fn current_user<'ctx>(ctx: &'ctx Context<'_>) -> Result<&'ctx User> {
    let d = data(ctx)?;
    match &d.user {
        Some(user) => Ok(user),
        None => Err(Error::InvalidPermissions.into()),
    }
}

pub fn data<'ctx>(ctx: &'ctx Context<'_>) -> Result<&'ctx ContextData> {
    ctx.data::<ContextData>()
}

pub fn model_keys(id: ID) -> ModelKeys {
    let model_keys = str::from_utf8(&base64::decode(id.to_string()).unwrap())
        .unwrap()
        .to_string()
        .split(":")
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let (model, keys_str) = model_keys.split_first().unwrap();
    let keys = keys_str
        .iter()
        .map(|pk_str| Uuid::parse_str(pk_str).unwrap())
        .collect::<Vec<Uuid>>();
    ModelKeys {
        model: model.to_owned(),
        keys,
    }
}

pub fn is_current_user(user: &User, ctx: &Context<'_>) -> Result<()> {
    let curr_user = current_user(ctx)?;
    if curr_user.uuid != user.uuid {
        return Err(Error::InvalidPermissions.into());
    }
    Ok(())
}

pub fn user_name_from_email(email: &str) -> String {
    email
        .split("@")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .replace(".", "_")
}
