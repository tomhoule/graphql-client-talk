// 1

fn call<Q>(
    &self,
    query: Q,
    variables: &Q::Variables
) -> Future<GraphQLQueryResult>
where Q: GraphQLQuery

let response = await!(client.call(RepositoriesIndex, &variables));

// 2

fn call<Q>(
    &self,
    variables: &Q::Variables
) -> Future<GraphQLQueryResult>
where Q: GraphQLQuery

let response = await!(client.call::<RepositoriesIndex>(&variables));
