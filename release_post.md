Today's release has by far the most improvements since graphql-client was initially release. This is all thanks to those who started using and contributing to the library.

## What graphql_client does, in short



## What is in 0.5

0.4 was ... months ago. Since then users and contributors have stepped up, reported bugs (which were fixed) and contributed a lot of awesome features. Here are a few highlights.

- Aliases in queries are now supported. Beyond the ergonomics, this is an important feature since it allows to query the same field on an object multiple times with different arguments, as shown in the [official guide](https://graphql.org/learn/queries/#aliases). Thanks a lot to @mathstuf!

- Configurable behaviour when a query uses a field marked as deprecated in the schema. By default, you will get warnings, but you can configure the library to fail to compile if a query uses a deprecated field, or completely disable the warnings.

- @h-michael has split the codegen out of the derive crate, to implement code generation via the CLI.

- For several classes of items that we generate, we only generate those that are actually used by the query. This way, you do not need to define mappings for _every_ scalar in the schema you are querying - even for small queries - anymore. This also improves compile times a lot in some scenarios. (#116)

- Top-level exported types have been renamed. Types no longer have a `GraphQL` prefix. e.g. `GraphQLQuery` -> `Query`, `GraphQLError` -> `Error`.

Those are just the highlights - there have been a lot of other significant performance and correctness improvements, and the corner-cases in the spec that we do not support are getting very rare.

Very special thanks to @h-michael, @mathstuf, @legneato and @scrogson for their work!

## Next steps

The GraphQL specification is nearly completely supported, but there are minor points that need work.

Here are a few exciting features we want to work on next:

- Currently code generation works with a custom derive, but we want to implement CLI and build script based flows as well. This should enable more code sharing, therefore reduced code size.

- Implement platform-specific client libraries to further reduce boilerplate. The recent publication of the [web-sys](web-sys) crate enables the creation a browser-native client built on top of it.

- A normalized cache implementation similar to what is implemented by apollo-client.

- Real query validation, with better error messages. We are exploring sharing code with [juniper](juniper) since the spec specifies validation in detail.
