use crate::lossy;


#[test]
fn identifier_should_work() {

    let test = lossy::IDParser::new().parse("abc").expect("Failed to parse");

    let test = r#"
        struct A {
            a: i64,
            b: i64,
        }
        struct B {
            c: i64,
            d: i64,
        }
        fn main() {
            let a = A { a: 1, b: 2 };
            let b = B { c: 3, d: 4 };
        }"#;

    let ast = lossy::ProgramParser::new()
        .parse(test)
        .expect("Failed to parse");
}
