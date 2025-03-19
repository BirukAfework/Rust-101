Certainly! Here's the entire content in Markdown format:

```markdown
# Top Common Rust Keywords

1. **`fn`** - Declares a function.  
   Example:
   ```rust
   fn main() {
       println!("Hello, World!");
   }
   ```

2. **`let`** - Declares a variable.  
   Example:
   ```rust
   let x = 5; // Immutable by default
   let mut y = 10; // Mutable variable
   y += 1;
   ```

3. **`mut`** - Makes a variable mutable.  
   Example:
   ```rust
   let mut x = 5;
   x = 10; // Allowed because x is mutable
   ```

4. **`const`** - Declares a compile-time constant.  
   Example:
   ```rust
   const PI: f32 = 3.14159;
   ```

5. **`static`** - Declares a global variable with a `'static` lifetime.  
   Example:
   ```rust
   static MAX_VALUE: i32 = 100;
   ```

6. **`if / else`** - Conditional branching.  
   Example:
   ```rust
   let x = 10;
   if x > 5 {
       println!("x is greater than 5");
   } else {
       println!("x is less than or equal to 5");
   }
   ```

7. **`match`** - Pattern matching (similar to `switch` in other languages).  
   Example:
   ```rust
   let x = 3;
   match x {
       1 => println!("One"),
       2 => println!("Two"),
       _ => println!("Other"),
   }
   ```

8. **`loop`** - Infinite loop.  
   Example:
   ```rust
   loop {
       println!("This will run forever!");
       break; // Exit the loop
   }
   ```

9. **`while`** - Conditional loop.  
   Example:
   ```rust
   let mut x = 0;
   while x < 5 {
       println!("x = {}", x);
       x += 1;
   }
   ```

10. **`for`** - Iterates over a range or collection.  
    Example:
    ```rust
    for i in 0..5 {
        println!("i = {}", i);
    }
    ```

11. **`break`** - Exits a loop.  
    Example:
    ```rust
    loop {
        println!("Breaking the loop!");
        break;
    }
    ```

12. **`continue`** - Skips the rest of the current loop iteration.  
    Example:
    ```rust
    for i in 0..5 {
        if i == 2 {
            continue; // Skip iteration when i == 2
        }
        println!("i = {}", i);
    }
    ```

13. **`return`** - Returns a value from a function.  
    Example:
    ```rust
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    ```

14. **`struct`** - Defines a custom data structure.  
    Example:
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 0 };
    ```

15. **`enum`** - Defines a type that can have multiple variants.  
    Example:
    ```rust
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let dir = Direction::Up;
    ```

16. **`impl`** - Implements functionality for a type.  
    Example:
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }
    ```

17. **`trait`** - Defines shared behavior (similar to interfaces in other languages).  
    Example:
    ```rust
    trait Speak {
        fn speak(&self);
    }
    struct Dog;
    impl Speak for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
    ```

18. **`pub`** - Makes an item (e.g., function, struct) publicly accessible.  
    Example:
    ```rust
    pub fn public_function() {
        println!("This is public!");
    }
    ```

19. **`use`** - Imports items from modules or crates.  
    Example:
    ```rust
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key", "value");
    ```

20. **`mod`** - Defines a module.  
    Example:
    ```rust
    mod my_module {
        pub fn my_function() {
            println!("Inside my_module!");
        }
    }
    my_module::my_function();
    ```

21. **`as`** - Renames an imported item or performs type casting.  
    Example:
    ```rust
    use std::fmt::Result as FmtResult;
    let x = 5.0 as i32;
    ```

22. **`type`** - Defines a type alias.  
    Example:
    ```rust
    type Kilometers = i32;
    let distance: Kilometers = 10;
    ```

23. **`where`** - Adds constraints to generic types.  
    Example:
    ```rust
    fn add<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        a + b
    }
    ```

24. **`async / await`** - Enables asynchronous programming.  
    Example:
    ```rust
    async fn fetch_data() -> String {
        "Data".to_string()
    }
    #[tokio::main]
    async fn main() {
        let data = fetch_data().await;
        println!("{}", data);
    }
    ```

25. **`dyn`** - Indicates a dynamic trait object.  
    Example:
    ```rust
    trait Animal {
        fn speak(&self);
    }
    struct Dog;
    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
    let animal: &dyn Animal = &Dog;
    animal.speak();
    ```

26. **`unsafe`** - Allows unsafe operations (e.g., raw pointers, dereferencing).  
    Example:
    ```rust
    unsafe fn dangerous() {
        let mut x = 5;
        let raw_ptr = &mut x as *mut i32;
        *raw_ptr = 10;
    }
    ```

27. **`extern`** - Links to external functions or libraries.  
    Example:
    ```rust
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 is {}", abs(-3));
    }
    ```

28. **`self / Self`** - Refers to the current instance or type.  
    Example:
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
        fn print(&self) {
            println!("({}, {})", self.x, self.y);
        }
    }
    ```

29. **`super`** - Refers to the parent module.  
    Example:
    ```rust
    mod parent {
        pub fn foo() {
            println!("Parent function!");
        }
    }
    mod child {
        use super::parent;
        pub fn bar() {
            parent::foo();
        }
    }
    ```

30. **`crate`** - Refers to the root of the current crate.  
    Example:
    ```rust
    use crate::my_module::my_function;
    ```
