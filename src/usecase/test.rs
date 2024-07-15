use crate::models;

pub fn get_obj() -> models::test::TestObj {
    models::test::TestObj {
        id: "test".to_string(),
        name: "test_name".to_string(),
        num: 1,
    }
}

pub fn create_obj(
    models::test::CreateTestObjInput { name, num }: models::test::CreateTestObjInput,
) -> Result<models::test::TestObj, std::io::Error> {
    let input = models::test::TestObj {
        id: "test_input".to_string(),
        name,
        num,
    };
    println!("{:?} created!", input);
    Ok(input)
}
