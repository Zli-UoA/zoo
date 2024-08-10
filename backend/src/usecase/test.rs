use crate::context::Context;
use crate::models::test::TestObj;

pub fn get_obj(ctx: &Context) -> TestObj {
    TestObj {
        id: ctx.env.clone(),
        name: ctx.env.clone(),
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
