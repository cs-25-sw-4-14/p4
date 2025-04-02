use std::fs;

#[test]
fn test_with_lzy_file() {
    let lzy_content = fs::read_to_string("test.lzy")
        .expect("Failed to read test.lzy file");

    println!("Content of test.lzy: {}", lzy_content);
}

