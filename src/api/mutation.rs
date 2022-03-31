use async_graphql::Object;
pub struct Mutation;

#[Object]
impl Mutation {
    async fn hello_world(&self) -> &str {
        "hello world"
    }
}