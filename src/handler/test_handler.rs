use crate::models;
use crate::usecase;
use async_graphql::Object;

#[derive(Default)]
pub struct TestQuery;

#[Object]
impl TestQuery {
    async fn get_obj(&self) -> models::test::TestObj {
        usecase::test::get_obj()
    }
}

#[derive(Default)]
pub struct TestMutation;

#[Object]
impl TestMutation {
    async fn create_obj(
        &self,
        input_obj: models::test::CreateTestObjInput,
    ) -> Result<models::test::TestObj, std::io::Error> {
        usecase::test::create_obj(input_obj)
    }
}
