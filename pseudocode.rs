// function which takes ownership over the String, and therefore frees it when it goes out of scope
void consume(String name) {
  println(name);
}

// function which takes ownership and returns it again
String consume_and_pass(String name) {
  name
}

// function which takes a reference
void test_reference(&String name) {
  println(name);
}

// function which takes a mutable reference and modifies it
void test_mut_reference(&Mut String name) {
  *name = "Mark";
}

// stack types are copied into functions
u1 get_gender(u1 is_man) {
  debug("{}", is_man);
  is_man
}

struct Person {
    String name;
    u1 age;
}

// enum PritorityDirection {
//   FrontToBack,
//   BackToFront
// }

void main() {
  // Initialize the String which is a heap-type and therefore allocated on the heap
  Mut String name = String::from("Hej");

  Mut String name = String::from("Hej");

  Mut Vector vector = Vector::new(i8);
 
  Result<(), Err> result = vector.try_push(10);

  is_ok()
  is_err()

  Mut PrioritizedVector vector = PrioritizedVector::new(i8, PritorityDirection::FrontToBack);
  Mut PrioritizedVector vector = PrioritizedVector::new(i8, PritorityDirection::BackToFront);

  Result<(), Err> result = vector.try_push(10);

  for i in 0..2 {
    vector.push(10).is_err()
  }

  if result.err() {
    Allocator.squash()
    Result<(), Err> result = vector.push(10)
                if .result.err() 
    // Allocator.aggressive_squash(2)
    Drop(vector)
  }

  Mut Array arr = Array::new(10, i8);

  // pass ownership over the name and receive it back
  Mut String other_name = consume_and_pass(name);

  // pass a mutable reference to the name
  test_mut_reference(&Mut name);

  // pass an immutable reference to the name
  test_reference(&name);

  // takes ownership of name and frees it when it goes out of scope of the function
  consume(name);

  &Mut String name2 = &Mut name;
  &Mut String name3 = &Mut name;

  consume(name);
  // custom types are allowed to simplify low-level management of individual bits
  // for example: u3, i1000
  u1 is_man = 0

  u4 wow = 10

  wow < is_man as u4 == true

  // if you call a function that returns something you must handle the return type
  // you can optionally ignore it with '_'
  u1 _ = get_gender(is_man);

  // filesystem access where it is possible to write and read from specific files
  // for reading and writing hardware IO through filesystem drivers
  File file = File::from("file.txt"):
  
  // write to the file
  file.write("sup");

}
