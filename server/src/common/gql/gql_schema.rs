use async_graphql::{Error, FieldResult, MergedObject, Object, SimpleObject};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, TestQuery);

#[derive(Default)]
struct UserQuery;

struct TestProp;


#[Object]
impl TestProp {
    async fn get_users(&self, int: i32) -> FieldResult<i32> {
        Ok(int)
    }
}

#[Object]
impl UserQuery {
    async fn users(&self) -> FieldResult<TestProp> {
        Ok(TestProp)
    }
}

#[derive(Default)]
struct TestQuery;

#[Object]
impl TestQuery {
    /// Returns the sum of a and b
    async fn add(&self, int: i32) -> FieldResult<i32> {
        Err(Error::new(String::from("test error")))
    }
}

#[derive(SimpleObject)]
struct Auth {
    result: i32,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn test_int(&self, int: i32) -> FieldResult<Auth> {
        let auth = Auth { result: int };
        Ok(auth)
    }
}
