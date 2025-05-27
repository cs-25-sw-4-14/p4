// --- 1. Basic Literals and Print/Debug ---
void test_literals() {
    debug(123); // IntLiteral
    debug("\n");
    debug(123.45); // FloatLiteral
    debug("\n");
    debug(true); // BoolLiteral
    debug("\n");
    debug(false); // BoolLiteral
    debug("\n");
    debug('a'); // CharLiteral
    debug("\n");
    debug("Hello, world!"); // StringLiteral
    debug("\n");
}

// --- 2. Variable Declarations and Assignments ---
void test_variables() {
    i64 a = 10;
    debug(a); // Expected: 10
    debug("\n");

    mut i64 b = 20;
    debug(b); // Expected: 20
    debug("\n");
    b = 30;
    debug(b); // Expected: 30
    debug("\n");

    triv bool c = true;
    debug(c); // Expected: true
    debug("\n");

    String s = "Rust";
    debug(&s); // Expected: Rust
    debug("\n");
    s = "Lossy";
    debug(s); // Expected: Lossy
    debug("\n");

    _ = 5 + 5; // Throwaway statement
}

// --- 3. Arithmetic Operations ---
void test_arithmetic() {
    debug(10 + 5);    // Expected: 15
    debug("\n");
    debug(20 - 7);    // Expected: 13
    debug("\n");
    debug(3 * 4);     // Expected: 12
    debug("\n");
    debug(25 / 5);    // Expected: 5
    debug("\n");
    debug(10 % 3);    // Expected: 1
    debug("\n");
    debug(-5);        // Expected: -5
    debug("\n");
    debug(1 + 2 * 3); // Expected: 7 (precedence)
    debug("\n");
    debug((1 + 2) * 3); // Expected: 9 (parentheses)
    debug("\n");
}

// --- 4. Comparison and Logical Operations ---
void test_comparisons() {
    debug(10 == 10); // Expected: true
    debug("\n");
    debug(10 != 5);  // Expected: true
    debug("\n");
    debug(10 > 5);   // Expected: true
    debug("\n");
    debug(5 < 10);   // Expected: true
    debug("\n");
    debug(10 >= 10); // Expected: true
    debug("\n");
    debug(5 <= 5);   // Expected: true
    debug("\n");

    debug(true && false); // Expected: false
    debug("\n");
    debug(true || false); // Expected: true
    debug("\n");
    debug(!true);         // Expected: false
    debug("\n");
}

// --- 5. If/Else If/Else Statements ---
void test_if_else() {
    i64 x = 10;
    if (x == 10) {
        debug("x is 10"); // Expected: x is 10
        debug("\n");
    } else if (x > 10) {
        debug("x is greater than 10");
        debug("\n");
    } else {
        debug("x is less than 10");
        debug("\n");
    }

    i64 y = 5;
    if (y > 10) {
        debug("y is greater than 10 (should not print)");
        debug("\n");
    } else if (y == 5) {
        debug("y is 5"); // Expected: y is 5
        debug("\n");
    } else {
        debug("y is something else (should not print)");
        debug("\n");
    }

    i64 z = 1;
    if (z > 5) {
        debug("z > 5 (should not print)");
        debug("\n");
    } else {
        debug("z is not greater than 5"); // Expected: z is not greater than 5
        debug("\n");
    }
}

// --- 6. While Loops, Break, and Continue ---
void test_while_loop() {
    mut i64 i = 0;
    while (i < 5) {
        if (i == 2) {
            i = i + 1;
            continue; // Skip debug(2)
        }
        if (i == 4) {
            debug("Breaking loop"); // Expected: Breaking loop
            debug("\n");
            break; // Exit loop
        }
        debug(i); // Expected: 0, 1, 3
        debug("\n");
        i = i + 1;
    }
}

// --- 7. Functions with Parameters and Return Values ---
i64 add(i64 a, i64 b) { // Parameters changed to: Type identifier
    return a + b;
}

void greet(String name) { // Parameters changed to: Type identifier
    debug("Hello, ");
    debug(name);
    debug("\n");
}

void test_functions() {
    debug(add(5, 7)); // Expected: 12
    debug("\n");
    greet("Lossy User"); // Expected: Hello, Lossy User
    debug("\n");
}

// --- 8. Structs ---
struct Point {
    i64 x;
    i64 y;
}

void test_structs() {
    mut Point p = Point { x: 10, y: 20 };
    debug(p.x); // Expected: 10
    debug("\n");
    debug(p.y); // Expected: 20
    debug("\n");

    p.x = 100;
    debug(p.x); // Expected: 100
    debug("\n");

    Point p2 = Point { y: 50, x: 25 }; // Order shouldn't matter with named fields
    debug(p2.x); // Expected: 25
    debug("\n");
    debug(p2.y); // Expected: 50
    debug("\n");
}

// --- 9. Arrays ---
void test_arrays() {
    arr i64 [3] my_array = [1, 2, 3];
    debug(my_array[0]); // Expected: 1
    debug("\n");
    debug(my_array[1]); // Expected: 2
    debug("\n");
    my_array[0] = 99;
    debug(my_array[0]); // Expected: 99
    debug("\n");

    arr bool [5] default_bool_array = [5; true]; // Initialize with size and default value
    debug(default_bool_array[0]); // Expected: true
    debug("\n");
    debug(default_bool_array[4]); // Expected: true
    debug("\n");
}

// --- 10. Vectors (Dynamic Arrays) ---
void test_vectors() {
    mut Vec<i32> my_vec = {10, 20, 30};
    debug(my_vec.get(0)); // Expected: 10
    debug("\n");
    debug(my_vec.get(2)); // Expected: 30
    debug("\n");

    my_vec.insert(1, 15); // Insert 15 at index 1
    debug(my_vec.get(0)); // Expected: 10
    debug("\n");
    debug(my_vec.get(1)); // Expected: 15
    debug("\n");
    debug(my_vec.get(2)); // Expected: 20 (shifted)
    debug("\n");

    my_vec.remove(0); // Remove element at index 0
    debug(my_vec.get(0)); // Expected: 15 (shifted)
    debug("\n");
    debug(my_vec.get(1)); // Expected: 20 (shifted)
    debug("\n");
}

// --- 11. References and Dereferencing ---
void test_references() {
    mut i64 val = 100;
    &mut i64 ref_val = &mut val;
    debug(*ref_val); // Expected: 100
    debug("\n");

    i64 val2 = *ref_val;

    val2 = 200; // Modify through reference
    debug(val);     // Expected: 200
    debug("\n");

    i64 other_val = 300;
    &i64 immut_ref_val = &other_val;
    debug(*immut_ref_val); // Expected: 300
    debug("\n");

    i64 immut = *ref_val;
  
    // immut = 400; // This should be a compile-time error
}

// --- 12. Type Casting ---
void test_type_casting() {
    i64 integer = 10;
    f64 float_from_int = integer as f64;
    debug(float_from_int); // Expected: 10.0
    debug("\n");

    f64 decimal = 15.7;
    i32 int_from_float = decimal as i32;
    debug(int_from_float); // Expected: 15 (truncation)
    debug("\n");

    i8 byte = 255 as i8; // Will likely wrap around or clip depending on backend
    debug(byte); // Expected: -1 (if 8-bit signed integer)
    debug("\n");

    // Example of explicit conversion to string (if supported by your codegen)
    // i32 num_to_string = 123;
    // String str_from_num = num_to_string as String;
    // debug(str_from_num); // Expected: "123"
}

// --- 13. File I/O (Placeholder - requires host interaction) ---
void test_file_io() {
    // This part is highly dependent on how your `read_file` and `write_file`
    // interact with the host environment (e.g., standard library functions
    // or system calls). For integration tests, you'd typically need to
    // set up dummy files in the temporary test directory.

    // write_file("Hello from Lossy!", "test_output.txt");
    // String file_content = read_file("test_output.txt");
    // debug(file_content); // Expected: Hello from Lossy!

    // For now, these are commented out as they need file system setup.
    // You would add specific host-side setup in your Rust integration test.
}

// --- Main function to run all tests ---
void main() {
    debug("--- Running test_literals ---\n");
    test_literals();
    debug("--- Running test_variables ---\n");
    test_variables();
    debug("--- Running test_arithmetic ---\n");
    test_arithmetic();
    debug("--- Running test_comparisons ---\n");
    test_comparisons();
    debug("--- Running test_if_else ---\n");
    test_if_else();
    debug("--- Running test_while_loop ---\n");
    test_while_loop();
    debug("--- Running test_functions ---\n");
    test_functions();
    debug("--- Running test_structs ---\n");
    test_structs();
    debug("--- Running test_arrays ---\n");
    test_arrays();
    debug("--- Running test_vectors ---\n");
    test_vectors();
    debug("--- Running test_references ---\n");
    test_references();
    debug("--- Running test_type_casting ---\n");
    test_type_casting();
    // debug("--- Running test_file_io ---\n");
    // test_file_io(); // Uncomment if file I/O is set up
    debug("--- All integration tests complete ---\n");
}
