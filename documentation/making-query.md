# Making queries

By default `Session` structure doesn't provide an API for making queries. Query functionality becomes enabled after importing one or few of following traits:

```rust
use cdrs_tokio::query::QueryExecutor;
```

This trait provides an API for making plain queries and immediately receiving responses.

```rust
use cdrs_tokio::query::PrepareExecutor;
```

This trait enables query preparation on the server. After query preparation it's enough to execute query via `cdrs_tokio::query::ExecExecutor` API providing a query ID returned by `cdrs_tokio::query::PrepareExecutor`

```rust
use cdrs_tokio::query::ExecExecutor;
```

This trait provides an API for query execution. `PrepareExecutor` and `ExecExecutor` APIs are considered in [next sections](./preparing-and-executing-queries.md).

```rust
use cdrs_tokio::query::BatchExecutor;
```

`BatchExecutor` provides functionality for executing multiple queries in a single request to Cluster. For more details refer to [Batch Queries](./batching-multiple-queries.md) section.

### `CdrsSession` trait

Each of traits enumerated beyond provides just a piece of full query API. They can be used independently one from another though. However, if the whole query functionality is needed in a program `use cdrs_tokio::cluster::CdrsSession` should be considered instead.

### `QueryExecutor` API

`QueryExecutor` trait provides various methods for immediate query execution.

```rust
session.query("INSERT INTO my.numbers (my_int, my_bigint) VALUES (1, 2)").unwrap();
```

`query` method receives a single argument which is a CQL query string. It returns `cdrs_tokio::error::Result` that in case of `SELECT` query can be mapped on corresponded Rust structure. See [CRUD example](../examples/crud_operations.rs) for details.

The same query could be made leveraging something that is called Values. It allows to have generic query strings independent of actual values.

```rust
#[macro_use]
extern crate cdrs;
//...

const INSERT_NUMBERS_QUERY: &'static str = "INSERT INTO my.numbers (my_int, my_bigint) VALUES (?, ?)";
let values = query_values!(1 as i32, 1 as i64);

session.query_with_values(INSERT_NUMBERS_QUERY, values).unwrap();
```

Here we've provided a generic query string for inserting numbers into `my.numbers` table. This query string doesn't have actual values hardcoded so exactly the same query can be used for multiple insert operations. Such sort of query strings can be used for Prepare-and-Execute operations when a query string is sent just once during Prepare step and then Execution operation is performed each time new values should be inserted. For more details see [Preparing and Executing](./preparing-and-executing-queries.md) section.

However, the full control over the query can be achieved via `cdrs_tokio::query::QueryParamsBuilder`:

```rust
use cdrs_tokio::query::QueryParamsBuilder;
use cdrs_tokio::consistency::Consistency;

let query_params = QueryParamsBuilder::new()
  .consistency(Consistency::Any)
  .finalize();
session.query_with_params("SELECT * FROM my.store", query_params).unwrap();
```

`QueryParamsBuilder` allows to precise all possible parameters of a query: consistency, values, paging properties and others. To get all parameters please refer to CDRS API [docs](https://docs.rs/cdrs/2.0.0-beta.1/cdrs/query/struct.QueryParamsBuilder.html).

Usually developers don't need to use `query_with_params` as almost all functionality is provided by such ergonomic methods as `query_with_values`, `pager` etc.

### Reference

1. `QueryParamsBuilder` API docs https://docs.rs/cdrs/2.0.0-beta.1/cdrs/query/struct.QueryParamsBuilder.html.

2. The Cassandra Query Language (CQL) http://cassandra.apache.org/doc/4.0/cql/.

3. [CDRS: Preparing and executing queries](./preparing-and-executing-queries.md)
