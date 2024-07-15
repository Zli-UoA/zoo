use crate::models::test::TestObj;

pub fn get_obj() -> TestObj {
    TestObj {
        id: "test".to_string(),
        name: "test_name".to_string(),
        num: 1,
    }
}

pub fn create_obj(name: String, num: usize) -> Result<TestObj, std::io::Error> {
    let input = TestObj {
        id: "test_input".to_string(),
        name,
        num,
    };
    println!("{:?} created!", input);
    Ok(input)
}
