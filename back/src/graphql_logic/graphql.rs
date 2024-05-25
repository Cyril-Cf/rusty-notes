use super::context::GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{EmptySubscription, FieldResult, RootNode};

// use super::data::Todos;
// use super::models::{CreateTodoInput, Todo};

// The root GraphQL query
pub struct Query;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    pub fn all_todos(context: &GraphQLContext) -> FieldResult<Vec<String>> {
        // // TODO: pass the GraphQLContext into the querying functions rather
        // // than a PgConnection (for brevity's sake)
        // let conn: &PgConnection = &context.pool.get().unwrap();

        // Todos::all_todos(conn)
        todo!()
    }
}

// The root GraphQL mutation
pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_todo(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    // pub fn mark_todo_as_done(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
    //     let conn: &PgConnection = &context.pool.get().unwrap();

    //     Todos::mark_todo_as_done(conn, id)
    // }
    // pub fn mark_todo_as_not_done(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
    //     let conn: &PgConnection = &context.pool.get().unwrap();

    //     Todos::mark_todo_as_not_done(conn, id)
    // }
}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}
