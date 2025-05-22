struct Point {
    i64 x;
    i64 y;
}

struct Circle {
    Point center;
    f64 radius;
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
    
    f64 pi = 3.14159;
    f64 precise_value = 2.7182818284590452353602;
    
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
        
        while (j < 5) {
            j = j + 1;
            
            if (i == 2 && j == 2) {
                
            }
            
            counter = counter + 1;
            
            if (j == 3) {
                
            }
        }
        
        if (i == 4) {
            
        }
    }
    
    Point origin = Point { x: 0, y: 0 };
    Point endpoint = Point { x: 100, y: 100 };

    Person person = Person {
        first_name: "Christoffer",
        last_name: "Johansen",
        age: 20,
        is_dork: true,
        location: Point {x: 50, y: 50}
    };   

    person.age = 21;

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

    Circle circle = Circle {
        center: Point {x: 50, y: 50},
        radius: 10.0
    };


    Point location = person.location;
    Point circle_center = circle.center;

    bool is_same_x = location.x == circle_center.x;

    String lol = read_file("lol");
    debug("a");

}
