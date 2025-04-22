struct Point {
    i64 x;
    i64 y;
}

struct Circle {
    Point center;
    fp64 radius;
}

struct Rectangle {
    Point top_left;
    Point bottom_right;
    String color;
}

struct Person {
    String first_name;
    String last_name;
    i64 age;
    bool is_dork;
    Point location;
}
void main() {
    
    triv i64 const_value = 10;
    mut i64 counter = 0;              
    triv mut i64 value = -100;        
    i64 large_number = 69420;
    
    fp64 pi = 3.14159;
    fp64 precise_value = 2.7182818284590452353602;
    
    bool is_true = true;
    bool is_false = false;
    
    String with_text = "Antlr? I barely know her!";
    String empty = "";
    
    i64 plus = 10 + 20;
    i64 minus = 50 - 30;
    i64 product = 6 * 7;
    i64 division = 100 / 5;
    i64 remainder = 10 % 3;
    
    i64 complex = (10 + 5) * 2 / (4 - 1);
    
    i64 negated = -value;
    bool inverted = !is_true;
    
    if (counter < (5 as u16)) {
        counter = (counter + 1 as u16) as u16;
    }
    
    if (value > (0 as i32)) {
        value = (value + 10 as i32) as i32;
    } else {
        value = 0 as i32;
    }
    
    if (counter > (100 as u16)) {
        counter = 100 as u16;
    } else if (counter > (50 as u16)) {
        counter = 50 as u16;
    } else if (counter > (10 as u16)) {
        counter = 10 as u16;
    } else {
        counter = 0 as u16;
    }
    
    if (is_true) {
        if (counter > (0 as u16)) {
            counter = (counter - 1 as u16) as u16;
        }
    }

    while (counter < (10 as u16)) {
        counter = (counter + 1 as u16) as u16;
    }
    
    while (counter < (20 as u16) && value < (0 as i32)) {
        counter = (counter + 1 as u16) as u16;
        value = (value + 10 as i32) as i32;
    }
    
    while (counter < (30 as u16)) {
        counter = (counter + 1 as u16) as u16;
        
        if (counter == 25) {
            
        }
    }
    
    mut i64 item_count = 0;
    while (item_count < 20) {
        item_count = item_count + 1;
        
        if (item_count % 2 == 0) {
            
        }
        
        i64 odd_value = item_count * 10;
    }
    
    mut i64 i = 0;
    while (i < 5) {
        i = i + 1;
        mut i64 j = 0;
        
        while (j < (5 as u8)) {
            j = (j + 1 as u8) as u8;
            
            if (i == 2 && j == 2) {
                
            }
            
            counter = counter + (1 as u16);
            
            if (j == 3) {
                
            }
        }
        
        if (i == 4) {
            
        }
    }
    
    Point origin = Point { x: 0 as i32, y: 0 as i32 };
    Point endpoint = Point { x: 100 as i32, y: 100 as i32 };

    Person person = Person {
        first_name: "Christoffer",
        last_name: "Johansen",
        age: 20,
        is_dork: true,
        location: Point {x: 50, y: 50}
    };   

    String full_name = person.first_name + " " + person.last_name;
    
    bool and_result = is_true && !is_false;
    bool or_result = is_true || is_false;
    
    bool complex_condition = ((counter > (10 as u16) && value < (0 as i32)) || (counter == (0 as u16)));
    
    bool is_equal = counter == value;
    bool not_equal = counter != value;
    bool is_greater = counter > value;
    bool is_less = counter < value;
    bool greater_equal = counter >= value;
    bool less_equal = counter <= value;

    Circle circle = Circle {
        center: Point {x: 50, y: 50},
        radius: 10.0
    };

    bool is_same_x = person.location.x == circle.center.x;
}

