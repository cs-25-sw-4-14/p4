// Memory Management Test Program

struct Person {
    i64 id;
    i64 age;
    String name;
}

struct Address {
    String street;
    i64 number;
    Person resident;
}

// Test function for creating and using structs with proper memory management
Person create_person(i64 id, i64 age, String name) {
    Person p = Person { id: id, age: age, name: name };
    return p;
}

// Test nested scopes and variable lifetimes
void test_scopes() {
    print("Testing variable scopes and cleanup...\n");
    
    // Outer scope
    i64 outer_var = 10;
    String outer_str = "Outer scope";
    
    // Create a block scope
    {
        print("  Entering inner scope 1\n");
        i64 inner_var = 20;
        String inner_str = "Inner scope 1";
        
        print("  inner_var = ");
        print(inner_var);
        print("\n");
        
        print("  outer_var = ");
        print(outer_var);
        print("\n");
        
        // Create a nested inner scope
        {
            print("    Entering nested scope\n");
            i64 nested_var = 30;
            String nested_str = "Nested scope";
            Person nested_person = Person { id: 999, age: 999, name: "Nested Person" };
            
            print("    nested_var = ");
            print(nested_var);
            print("\n");
            
            print("    inner_var = ");
            print(inner_var);
            print("\n");
            
            print("    nested_person.name = ");
            print(nested_person.name);
            print("\n");
            print("    Exiting nested scope\n");
            // nested_person should be freed here
            // nested_var and nested_str should be cleaned up here
        }
        
        print("  Still in inner scope 1\n");
        // inner_var and inner_str are still accessible here
        print("  Exiting inner scope 1\n");
        // inner_var and inner_str should be cleaned up here
    }
    
    // Create another block scope
    {
        print("  Entering inner scope 2\n");
        i64 another_var = 40;
        Person inner_person = create_person(101, 25, "Inner Person");
        
        print("  inner_person.name = ");
        print(inner_person.name);
        print("\n");
        
        print("  Exiting inner scope 2\n");
        // inner_person should be freed here
        // another_var should be cleaned up here
    }
    
    print("Back in outer scope\n");
    print("outer_var = ");
    print(outer_var);
    print("\n");
    
    // outer_var and outer_str are still accessible here
}

// Test early returns and how they affect memory management
void test_early_return(i64 condition) {
    print("Testing early returns...\n");
    
    String str1 = "String before condition";
    print(str1);
    print("\n");
    
    if (condition > 5) {
        print("Condition > 5, returning early\n");
        Person temp_person = Person { id: 500, age: 50, name: "Early Return Person" };
        print("Created temp_person: ");
        print(temp_person.name);
        print("\n");
        return; // temp_person and str1 should be freed here
    }
    
    String str2 = "String after condition";
    Person another_person = Person { id: 600, age: 60, name: "Normal Return Person" };
    
    print("Condition <= 5, continuing to end\n");
    print(str2);
    print("\n");
    print(another_person.name);
    print("\n");
    
    // str1, str2, and another_person should be freed here
}

// Main test function
void main() {
    print("Memory Management Test Program\n");
    print("=============================\n\n");
    
    // Test 1: Simple struct creation and access
    print("Test 1: Simple struct creation\n");
    Person p1 = Person { id: 1, age: 30, name: "John Doe" };
    print("Created person: ");
    print(p1.name);
    print(", age: ");
    print(p1.age);
    print("\n\n");
    
    // Test 2: Nested struct creation
    print("Test 2: Nested struct creation\n");
    Address addr = Address {
        street: "Main Street",
        number: 123,
        resident: Person { id: 2, age: 28, name: "Jane Smith" }
    };
    print("Created address: ");
    print(addr.street);
    print(" ");
    print(addr.number);
    print(", resident: ");
    print(addr.resident.name);
    print("\n\n");
    
    // Test 3: Function that returns a struct
    print("Test 3: Function returning a struct\n");
    Person p2 = create_person(3, 35, "Alice Johnson");
    print("Function returned person: ");
    print(p2.name);
    print(", age: ");
    print(p2.age);
    print("\n\n");
    
    // Test 4: Variable scopes and cleanup
    print("Test 4: Variable scopes\n");
    test_scopes();
    print("\n");
    
    // Test 5: Early returns
    print("Test 5: Early returns\n");
    print("Case A: Condition = 10\n");
    test_early_return(10);
    print("\nCase B: Condition = 3\n");
    test_early_return(3);
    print("\n");
    
    print("All tests complete!\n");
}