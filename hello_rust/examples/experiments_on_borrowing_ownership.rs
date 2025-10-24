fn main() {
    ownership();
    structs_and_enums();
    structs_with_functions();
    basic_enum();
    enum_with_data();
    enum_with_multiple_data_types();
    stack_implementation();
}

fn ownership() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // Value borrowed here has been moved , so it will give the compile time error. The ownership of s1 has been moved to s2.
    // println!("It will give error {}",s1);
    println!("No error {}", s2);

    // We can have one mutable reference and any number of immutable reference to the data.
    // Having only one mutable reference prevents the multiple updates at a time on the same data.

    let s = String::from("Rustacean");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2); // ✅ Works fine
    // Both are read-only borrows

    let mut strr = String::from("Hello");
    let r3 = &mut strr;
    r3.push_str(", world!"); // ✅ OK
    println!("{}", r3);

    // In rust We can't have mutable and immutable reference at once because one can mutate the data while others are reading it , that's data race.
    let mut s8 = String::from("Rust");

    let r4 = &s8;
    let r5 = &s8;
    // let r6 = &mut s8; // ❌ Error: cannot borrow `s` as mutable because it’s already borrowed as immutable

    // println!("{}, {}, {}", r4, r5, r6);

    // How rust will solve this problem
    // Rust allow the mutable and immutable reference only in the condition of non - overlapping scopes.
    let mut s7 = String::from("Rust");

    {
        let r1 = &s; // Immutable borrow
        println!("{}", r1);
    } // r1 goes out of scope here ✅

    let r7 = &mut s7; // Now mutable borrow allowed
    r7.push_str(" language");
    println!("{}", r7);

    // Pointers in Rust can't dangle , like it can't point to the freed memory.
}

fn structs_and_enums() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("pragyan@gmail.com");
    println!("User email is : {}", user1.email);
}

fn structs_with_functions() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            // &self means this function borrows an immutable reference to the struct.
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("Area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

// Enums :-> When we need multiple possible variants.
fn basic_enum() {
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let move_dir = Direction::North;

    match move_dir {
        Direction::North => println!("Going up!"),
        Direction::South => println!("Going down!"),
        Direction::East => println!("Heading right!"),
        Direction::West => println!("Heading left!"),
    }
}

fn enum_with_data() {
    enum Transport {
        Car(String),
        Bicycle(String),
    }

    fn check_garage() -> Transport {
        let car_model = String::from("Tesla Model Y");
        Transport::Car(car_model)
    }

    let available_transport = check_garage();
    // Pattern matching (match) is a compile-time checked switch-case — you must handle all cases.
    match available_transport {
        Transport::Car(model) => {
            println!("You're taking the car today! It's a {}.", model);
        }
        Transport::Bicycle(brand) => {
            println!("You're biking today on your {}.", brand);
        }
    }
}

fn enum_with_multiple_data_types() {
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }

    fn area(shape: Shape) -> f64 {
        match shape {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rectangle(l, b) => l * b,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle(4.0, 6.0);
    let t = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Circle area: {}", area(c));
    println!("Rectangle area: {}", area(r));
    println!("Triangle area: {}", area(t));
}

fn stack_implementation() {
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            // Here we are removing the element from the stack so the element's ownership  should also be moved out of the stack.
            self.items.pop()
        }

        fn peek(&self) -> Option<&T> {
            // Here we are returning the reference to the item because the element is still inside the stack so the ownership must be intact with the stack only.
            self.items.last()
        }

        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }
    let mut my_stack = Stack::new();
    my_stack.push(10);
    my_stack.push(20);

    println!("Top item: {:?}", my_stack.peek()); // Some(20)
    println!("Is empty: {}", my_stack.is_empty());

    println!("Popped: {:?}", my_stack.pop()); // Some(20)
    println!("Popped: {:?}", my_stack.pop()); // Some(10)
    println!("Popped: {:?}", my_stack.pop()); // None
}
