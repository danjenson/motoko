use crate::{
    context_data::ContextData, mutation::MutationRoot, query::QueryRoot,
};
use async_graphql::{EmptySubscription, Schema};

#[derive(Clone)]
pub struct State {
    pub schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    pub context_data: ContextData,
}
