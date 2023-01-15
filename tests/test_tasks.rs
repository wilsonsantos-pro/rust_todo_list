use tasks;

pub fn setup() {
    tasks::init().unwrap();
}

#[test]
fn test_list_tasks() {
    setup();
    let result = tasks::list_tasks();
    let expected = ();
    match result {
        Ok(result) => assert_eq!(result, expected),
        Err(e) => panic!("{}", e),
    }
}
