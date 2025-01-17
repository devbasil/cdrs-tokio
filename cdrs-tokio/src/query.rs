mod batch_query_builder;
mod prepared_query;
mod query_flags;
mod query_params;
mod query_params_builder;
mod query_values;
pub(crate) mod utils;

pub use crate::query::batch_query_builder::{BatchQueryBuilder, QueryBatch};
pub use crate::query::prepared_query::PreparedQuery;
pub use crate::query::query_flags::QueryFlags;
pub use crate::query::query_params::QueryParams;
pub use crate::query::query_params_builder::QueryParamsBuilder;
pub use crate::query::query_values::QueryValues;

/// Structure that represents CQL query and parameters which will be applied during
/// its execution
#[derive(Debug, Default)]
pub struct Query {
    pub query: String,
    pub params: QueryParams,
}
