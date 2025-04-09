struct Point {
    i32 x;
    i32 y;
}

struct Circle {
    Point center;
    f32 radius;
}

struct Rectangle {
    Point top_left;
    Point bottom_right;
    String color;
}

struct Person {
    String first_name;
    String last_name;
    u8 age;
    bool is_dork;
    Point location;
}

Void main() {
    
    triv u8 const_value = 10;
    mut u16 counter = 0;              
    triv mut i32 value = -100;        
    u64 large_number = 69420;
    
    f32 pi = 3.14159;
    f64 precise_value = 2.7182818284590452353602;
    
    bool is_true = true;
    bool is_false = false;
    
    String with_text = "Antlr? I barely know her!";
    String empty = "";
    
    i32 plus = 10 + 20;
    i32 minus = 50 - 30;
    i32 product = 6 * 7;
    i32 division = 100 / 5;
    i32 remainder = 10 % 3;
    
    i32 complex = (10 + 5) * 2 / (4 - 1);
    
    i32 negated = -value;
    bool inverted = !is_true;
    
    
    if (counter < 5) {
        counter = counter + 1;
    }
    
    if (value > 0) {
        value = value + 10;
    } else {
        value = 0;
    }
    
    if (counter > 100) {
        counter = 100;
    } else if (counter > 50) {
        counter = 50;
    } else if (counter > 10) {
        counter = 10;
    } else {
        counter = 0;
    }
    
    if (is_true) {
        if (counter > 0) {
            counter = counter - 1;
        }
    }

    while (counter < 10) {
        counter = counter + 1;
    }
    
    while (counter < 20 && value < 0) {
        counter = counter + 1;
        value = value + 10;
    }
    
    while (counter < 30) {
        counter = counter + 1;
        
        if (counter == 25) {
            break;
        }
    }
    
    mut u8 item_count = 0;
    while (item_count < 20) {
        item_count = item_count + 1;
        
        if (item_count % 2 == 0) {
            continue;
        }
        
        i32 odd_value = item_count * 10;
    }
    
    mut u8 i = 0;
    while (i < 5) {
        i = i + 1;
        mut u8 j = 0;
        
        while (j < 5) {
            j = j + 1;
            
            if (i == 2 && j == 2) {
                continue;
            }
            
            counter = counter + 1;
            
            if (j == 3) {
                break;
            }
        }
        
        if (i == 4) {
            continue;
        }
    }
    
    Point origin = Point {x: 0, y: 0};
    Point endpoint = Point {x: 100, y: 100};

    
    String full_name = person.first_name + " " + person.last_name;
    
    bool and_result = is_true && !is_false;
    bool or_result = is_true || is_false;
    
    bool complex_condition = (counter > 10 && value < 0) || (counter == 0);
    
    bool is_equal = counter == value;
    bool not_equal = counter != value;
    bool is_greater = counter > value;
    bool is_less = counter < value;
    bool greater_equal = counter >= value;
    bool less_equal = counter <= value;
    
    bool is_same_x = person.location.x == circle.center.x;
}

