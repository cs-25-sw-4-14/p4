void test_literals() {
    debug(123);
    debug("\n");
    debug(123.45);
    debug("\n");
    debug(true);
    debug("\n");
    debug(false);
    debug("\n");
    debug('a');
    debug("\n");
    debug("Hello, world!");
    debug("\n");
}

void test_variables() {
    i64 a = 10;
    debug(a);
    debug("\n");

    mut i64 b = 20;
    debug(b);
    debug("\n");
    b = 30;
    debug(b);
    debug("\n");

    triv bool c = true;
    debug(c);
    debug("\n");

    String s = "Rust";
    debug(&s);
    debug("\n");
    s = "Lossy";
    debug(s);
    debug("\n");

    _ = 5 + 5;
}

void test_arithmetic() {
    debug(10 + 5);
    debug("\n");
    debug(20 - 7);
    debug("\n");
    debug(3 * 4);
    debug("\n");
    debug(25 / 5);
    debug("\n");
    debug(10 % 3);
    debug("\n");
    debug(-5);
    debug("\n");
    debug(1 + 2 * 3);
    debug("\n");
    debug((1 + 2) * 3);
    debug("\n");
}

void test_comparisons() {
    debug(10 == 10);
    debug("\n");
    debug(10 != 5);
    debug("\n");
    debug(10 > 5);
    debug("\n");
    debug(5 < 10);
    debug("\n");
    debug(10 >= 10);
    debug("\n");
    debug(5 <= 5);
    debug("\n");
    debug(true && false);
    debug("\n");
    debug(true || false);
    debug("\n");
    debug(!true);
    debug("\n");
}

void test_if_else() {
    i64 x = 10;
    if (x == 10) {
        debug("x is 10");
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
        debug("y is 5");
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
        debug("z is not greater than 5");
        debug("\n");
    }
}

void test_while_loop() {
    mut i64 i = 0;
    while (i < 5) {
        if (i == 2) {
            i = i + 1;
            continue;
        }
        if (i == 4) {
            debug("Breaking loop");
            debug("\n");
            break;
        }
        debug(i);
        debug("\n");
        i = i + 1;
    }
}

i64 add(i64 a, i64 b) {
    return a + b;
}

void greet(String name) {
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

struct Point {
    i64 x;
    i64 y;
}

void test_structs() {
    mut Point p = Point { x: 10, y: 20 };
    debug(p.x);
    debug("\n");
    debug(p.y);
    debug("\n");

    p.x = 100;
    debug(p.x);
    debug("\n");

    Point p2 = Point { y: 50, x: 25 };
    debug(p2.x);
    debug("\n");
    debug(p2.y);
    debug("\n");
}

void test_arrays() {
    arr i64 [3] my_array = [1, 2, 3];
    debug(my_array[0]);
    debug("\n");
    debug(my_array[1]);
    debug("\n");
    my_array[0] = 99;
    debug(my_array[0]);
    debug("\n");

    arr bool [5] default_bool_array = [5; true];
    debug(default_bool_array[0]);
    debug("\n");
    debug(default_bool_array[4]);
    debug("\n");
}

void test_vectors() {
    mut Vec<i32> my_vec = {10, 20, 30};
    debug(my_vec.get(0));
    debug("\n");
    debug(my_vec.get(2));
    debug("\n");

    my_vec.insert(1, 15);
    debug(my_vec.get(0));
    debug("\n");
    debug(my_vec.get(1));
    debug("\n");
    debug(my_vec.get(2));
    debug("\n");

    my_vec.remove(0);
    debug(my_vec.get(0));
    debug("\n");
    debug(my_vec.get(1));
    debug("\n");
}

void test_references() {
    mut i64 val = 100;
    &mut i64 ref_val = &mut val;
    debug(*ref_val);
    debug("\n");

    i64 val2 = *ref_val;

    val2 = 200;
    debug(val);
    debug("\n");

    i64 other_val = 300;
    &i64 immut_ref_val = &other_val;
    debug(*immut_ref_val);
    debug("\n");

    i64 immut = *ref_val;
  
    // immut = 400;
}

void test_type_casting() {
    i64 integer = 10;
    f64 float_from_int = integer as f64;
    debug(float_from_int);
    debug("\n");

    f64 decimal = 15.7;
    i32 int_from_float = decimal as i32;
    debug(int_from_float);
    debug("\n");

    i8 byte = 255 as i8;
    debug(byte);
    debug("\n");
}


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
    debug("--- All integration tests complete ---\n");
}
