#![allow(unused)]
use crate::{
    ast::{Statement, TopLevelDefinition, Type},
    lossy,
};

#[test]
fn integration_test() {
    let source_code = r#"
        struct Point {
            fp32 x;
            fp32 y;
        }

        struct Rectangle {
            Point top_left;
            Point bottom_right;
        }

        fp32 calculate_area(Rectangle rect) {
            fp32 width = rect.bottom_right.x - rect.top_left.x;
            fp32 height = rect.bottom_right.y - rect.top_left.y;
            return width * height;
        }

        bool is_inside(Point p, Rectangle rect) {
            return p.x >= rect.top_left.x && 
                   p.x <= rect.bottom_right.x &&
                   p.y >= rect.top_left.y &&
                   p.y <= rect.bottom_right.y;
        }

        u32 main() {
            Point p1 = Point { x: 1.0, y: 2.0 };
            Point p2 = Point { x: 5.0, y: 7.0 };
            Rectangle rect = Rectangle { top_left: p1, bottom_right: p2 };
            
            // Function calls
            fp32 area = calculate_area(rect);
            
            u32 int_area = area;           
            
            if (int_area > 20) {
                Point test_point = Point { x: 3.0, y: 4.0 };
                if (is_inside(test_point, rect)) {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                triv mut u32 counter = 0;
                while (counter < 5) {
                    counter = counter + 1;
                }
                return counter;
            }
        }
    "#;

    let parsed_program = lossy::ProgramParser::new()
        .parse(source_code)
        .expect("Failed to parse program");

    assert_eq!(
        5,
        parsed_program.definitions.len(),
        "Program should have 5 top-level definitions"
    );

    let mut found_struct_point = false;
    let mut found_struct_rectangle = false;
    let mut found_fn_calculate_area = false;
    let mut found_fn_is_inside = false;
    let mut found_fn_main = false;

    for def in &parsed_program.definitions {
        match def {
            TopLevelDefinition::Struct(struct_def) => {
                if struct_def.name == "Point" {
                    found_struct_point = true;
                    assert_eq!(
                        2,
                        struct_def.fields.len(),
                        "Point struct should have 2 fields"
                    );
                } else if struct_def.name == "Rectangle" {
                    found_struct_rectangle = true;
                    assert_eq!(
                        2,
                        struct_def.fields.len(),
                        "Rectangle struct should have 2 fields"
                    );
                }
            }
            TopLevelDefinition::Function(fn_def) => match fn_def.name.as_str() {
                "calculate_area" => {
                    found_fn_calculate_area = true;
                    assert_eq!(
                        Some(Type::FP32),
                        fn_def.return_type,
                        "calculate_area should return fp32"
                    );
                    assert_eq!(
                        1,
                        fn_def.params.len(),
                        "calculate_area should have 1 parameter"
                    );
                }
                "is_inside" => {
                    found_fn_is_inside = true;
                    assert_eq!(
                        Some(Type::Bool),
                        fn_def.return_type,
                        "is_inside should return bool"
                    );
                    assert_eq!(2, fn_def.params.len(), "is_inside should have 2 parameters");
                }
                "main" => {
                    found_fn_main = true;
                    assert_eq!(
                        Some(Type::U32),
                        fn_def.return_type,
                        "main should return u32"
                    );
                    assert_eq!(0, fn_def.params.len(), "main should have 0 parameters");

                    assert!(
                        fn_def.body.len() > 3,
                        "main function should have multiple statements"
                    );

                    assert!(
                        fn_def.body.iter().any(|stmt| match stmt {
                            Statement::If(_, _) => true,
                            _ => false,
                        }),
                        "main should contain an if statement"
                    );
                }
                _ => {}
            },
        }
    }
    assert!(found_struct_point, "Point struct not found");
    assert!(found_struct_rectangle, "Rectangle struct not found");
    assert!(found_fn_calculate_area, "calculate_area function not found");
    assert!(found_fn_is_inside, "is_inside function not found");
    assert!(found_fn_main, "main function not found");
}
