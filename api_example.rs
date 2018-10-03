#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/my_query.graphql",
)]
pub struct MyQuery;

// magic happens

pub mod my_query {
    // The shape of the response
    struct ResponseData { ... }
    // The variables needed by the query
    struct Variables { ... }

    // A convenience trait
    impl graphql_client::GraphQLQuery for MyQuery {
        fn build_query(variables: Variables) -> RequestBody {
            ...
        }
    }
}

let variables = my_query::Variables { ... };
let request_body = MyQuery::build_query(variables);

let client = reqwest::Client::new();
let mut res = client.post("/graphql").json(&request_body).send()?;
let response_body: Response<my_query::ResponseData> = res.json()?;
