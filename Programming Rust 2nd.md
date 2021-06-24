### 2. A Tour Of Rust

- if a function body ends with an expression which is not followed by a semicolon, that's a function return value
  ```rust
  fn demo() -> i32 { 1 }
  ```
- block surrounded by curly braces can function as an expression and the last expression without semicolon can be treated as its value
  ```rust
  let x = { 1 };
  ```

### Fundamental Types

- soundness type system
  - type system is sound implies all of the type-checked programs are correct (i.e. no [false negative](https://en.wikipedia.org/wiki/False_positives_and_false_negatives#False_negative_error)).
  - type system is complete implies all of the correct programs can be accepted by type-checked (i.e. no [false positive](https://en.wikipedia.org/wiki/False_positives_and_false_negatives#False_positive_error)).
  - [soundness vs completeness](https://en.wikipedia.org/wiki/Soundness#Relation_to_completeness): soundness means a program passed type check is valid (run without any errors) while completeness means a valid program can be type-checked.
- `isize/usize` has the same size of an address on the machine (32 or 64 bits).
- `char` is unicode character, 32 bits wide.
- if an integer literal lacks a type suffix, its type is determined when there is a clue (stored in a variable of a particular type, passed to a function, or return to a function ...). If there are multiple possibilities, i32 will be picked (same for f64).
  ```rust
  println!("{}", (-4).abs());
  ```
  doesn't compile since Rust doesn't know which integer type a value belongs to, default i32 applies only if the type is still ambiguous after all method calls have been resolved which is too late here.
- integer downcast from N to M bits (N >> M) is doing the truncation of N - M most significant bits.
- signed integer overflow is undefined in C/C++, Rust introduces _Checked, Wrapping, Saturating and Overflowing_ to handle in the way you want. By default, doing that will cause panic in debug build and modulo wrap in release build.
- function returns no value has type of `()` (unit type)
  ```rust
  fn foo(x: i32, y: i32);
  ==
  fn foo(x: i32, y: i32) -> ();
  ```
- Rust permits an extra trailing comma in function arguments, arrays, struct, enum definition ... and it has no meaning
  ```rust
  (&str, i32, ) == (&str, i32)
  ```
- Rust borrow rules: you either have any number of immutable references or only one mutable reference at a given time (lifetime of references)
- Slices `&[T]` are always passed by reference and are converted automatically from vector and array
  ```rust
  let v: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];
  let a: [f64; 4] =     [0.0, -0.707, -1.0, -0.707];
  ---
  let sv: &[f64] = &v;
  let sa: &[f64] = &a;
  ```
  ![slice](https://i.imgur.com/kM9W5Qh.png)
- C/C++ doesn't support multiline string while Rust does.
  ```rust
  println!("hello world, Rust
    the weather is good today");
  ```
  the output will include newline and space at the beginning of the second line

### 4. Ownership and Moves

- there are two popular ways (using garbage collection or explicitly allocate/free) to manage memory while running. Rust uses a different approach, memory is managed through ownership which the compiler checks at compile-time.
- ownership rules:
  - each value in Rust has a variable called the owner.
  - there can be only one owner at a time.
  - when the owner goes out of scope, the value is dropped.
- for most types (except _Copy_ trait), operations like assigning a value to a variable, passing it to a function, or returning it from a function don't copy the value, _move_ is used.
  ```rust
  let s = vec!["udon", "ramen", "soba"];
  let t = s;
  println!("{:?}", t); // cannot use s, since value is moved to u
  ```
  ![move](https://i.imgur.com/BiEl2by.png)
- if you move a value into a variable that was already initialized, the prior value is dropped.
  ```rust
  let mut s = String::from("hello");
  s = String::from("world"); // string hello is dropped
  ---
  let mut s = String::from("hello");
  let t = s;
  s = String::from("world"); // nothing is dropped here
  ```
- if there is a possibility a value is moved, it is considered moved
  ```rust
  fn foo(c: u32) {
    let x = vec![10, 20, 30];
    if c > 10 {
        baz(x);
    }
    baz(x); // error: x is uninitialized here since x might be moved in if
  }
  ```
- any type that needs to do something special when a value is dropped cannot be _Copy_ (whole value is on stack)

### 5. References

- if there is an immutable reference, the owner cannot modify its value. If there is a mutable reference, owner cannot be used (until the mutable reference goes away).
- to a function:
  - pass by value <-> move the ownership
  - pass by reference <-> borrow from the owner
- [`.` operator](https://doc.rust-lang.org/nomicon/dot-operator.html) performs a lot of magic to convert types including auto-reference, auto-dereference ...
- Once C++ reference has been initialized, there is no way to change while in Rust reference can be changed.
  ```rust
  let x = 10;
  let y = 20;
  let mut z = &x;
  z = &y;
  ```
- reference to references.
  ```rust
  struct Point { x: i32, y: i32 }
  let point = Point { x: 1000, y: 729 };
  let r: &Point = &point;
  let rr: &&Point = &r;
  let rrr: &&&Point = &rr;
  ```
  ![reference](https://i.imgur.com/KelOvgm.png)
- comparing references performs on their final target. Using `std::ptr:eq`, if you want to compare their addresses
  ```rust
  let x = 10;
  let y = 10;
  let rx = &x;
  let ry = &y;
  let rrx = &rx;
  let rry = &ry;
  assert!(rrx <= rry);
  assert!(rrx == rry);
  assert!(!std::ptr::eq(rx, ry));
  ```
- references are never null. You can reference to the value of any sort of expression.
  ```rust
  let x = &1000; // anonymous variable is created to hold a value
  ```
- every reference (to be correct, variable) has a lifetime which is used by Rust to prevent a dangling pointer.
  ```rust
  let x;
  {
    let y = 10;
    x = &y; // lifetime of &y from this point to end of block
  }
  println!("{}", x); // error
  ```
- you can explicity specify lifetime in function parameters and return.
  ```rust
  fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {}
  ```
- each kind of reference affects what we can do with other values in the same ownership tree.
  ![tree](https://i.imgur.com/52OG5Au.png)

### 6. Expressions

- Rust is expression language (most statements in C are expressions in Rust).
- value of the block is the last expression without a semicolon.
  ```rust
  {
    let x = 10;
    x  // value of this block
  }
  ```
- `if` without `else` must always return `()` and all blocks of `if` expression must produce values of the same type (similar to `match`).
  ```rust
  let suggested_pet =
    if with_wings { Pet::Buzzard } else { Pet::Hyena };  // ok
  let favorite_number =
      if user.is_hobbit() { "eleventy-one" } else { 9 };  // error
  let best_sports_team =
      if is_hockey_season() { "Predators" };  // error
  ```
- `fn` is declared inside a block, its scope is the entire block (outside cannot see it) and it cannot access local variables.
- a condition (in `if`, `while` ...) must be an expression of `bool` (Rust doesn't implicitly convert to bool).
- loops are also expressions in Rust (`while`, `for` produces `()`). `break` can be used with label and value while `continue` only can be used with a label
  ```rust
  let foo = 'outer: loop {
    break 'outer 10;
  }
  ```
- unlike C, Rust differentitate between `()` (unit type) and `!` (never returns).
- `a..b` is end-exclusive range (not include b) while `a..=b` is end-inclusive range.
- unlike C, `%` can be used on floating-point numbers
  ```rust
  let x = 12.5 % 10; // 2.5
  ```
- values of type `bool`, `char` or, C-like `enum` may be cast to any integer type. The opposite direction is not allowed for example cannot cast `u16` to `char` because some `u16` values cannot be presented in `char` (`0xd800`). We can use a standard method `std::char::from_u32()` which returns `Option<char>`

### 7. Error Handling

- There are two kinds of error handling: panic and `Result`.
- panic is safe (catch before it happens), it doesn't violate any of Rust's safety rules since stack (including heap segments linked to variables) is cleanup -> there is no dangling pointer. Panic is like `RuntimeException` in C++.
- second panic happens during the cleanup of the first panic causes fatal -> thread is aborted. You can also config panic behavior like `-C panic=abort` (abort in the first panic).
- there is a shortcut for handling `Result` (like `unwrap/expect`) and error propagation (`?`).

### 8. Crates and Modules

- There are two kinds of crate: binary or library. You can either specify or let Rust figure it out by looking at `src/lib.rs` or `src/main.rs`.
- program can mix crates written in different editions since edition only affects how source code is construed.
- modules can be nested and be specified with `pub(super)/pub(in <path>)` to make them visible to a specific parent or its descendants.
- a path (similar to filesystem) can take two forms:
  - absolute path starts a crate root like `crate`.
  - relative path starts from a current module like `self/super`.
  ```rust
  mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
  }
  pub fn eat_at_restaurant() {
      // Absolute path
      crate::front_of_house::hosting::add_to_waitlist();
      // Relative path
      front_of_house::hosting::add_to_waitlist();
  }
  ```
- struct's fields, even private fields, are accessible in the module and its submodules where the struct is declared. Outside the module, only public fields are accessible.
- `const` is similar to C `#define`, `static` is similar to C `static`
  ```rust
  pub const FOO: i32 = 10;
  pub static BAZ: &str = "hello world";
  ```
- tests are ordinary functions marked with the `#[test]` attribute. You can either use `assert!` family macros to validate or return `Result<(), E>`.
- tests are compiled conditionally, `cargo build` skips the testing code.
- there are 3 styles of testings:
  - unit testing lives right alongside your code.
  - integration testing, files (each file is compiled as a separate crate) in `tests` directory.
  - documentation testing, code block in your document. `rustdoc` stores each code sample in a separate file, adding boilerplate code to produce programs. You disable if needed via `no_run/ignore`.
  ```rust
  /// Return true if two ranges overlap.
  ///
  ///     assert_eq!(ranges::overlap(0..7, 3..10), true);
  ///     assert_eq!(ranges::overlap(1..5, 101..105), false);
  ///
  /// If either range is empty, they don't count as overlapping.
  ///
  ///     assert_eq!(ranges::overlap(0..0, 0..10), false);
  ///
  ---- // temporary file 1
  use ranges;
  fn main() {
      assert_eq!(ranges::overlap(0..7, 3..10), true);
      assert_eq!(ranges::overlap(1..5, 101..105), false);
  }
  ---- // temporary file 2
  use ranges;
  fn main() {
      assert_eq!(ranges::overlap(0..0, 0..10), false);
  }
  ```
- cargo uses semantic versioning, lock mechanism, and workspace (probably like `yarn`).

### 8. Structs

- There are 3 kinds of struct types: named-field, tuple-like and unit-like.
- struct and its fields are private by default. Visibility is for different modules, creating a struct value requires all the struct's fields are visible.
  ```rust
  mod Foo {
    #[derive(Debug)]
    pub struct Point {
        pub x: i32,
        y: i32,         // private
    }
  }
  pub struct Distance {
      meter: i32,
  }
  fn main() {
      let p = Foo::Point { x: 10, y: 20 }; // y is private, not visible since Point and main belong to different modules
      let d = Distance { meter: 100 };     // meter is private, Distance and main belong to the same module
  }
  ```
- not like the spread operator in javascript, `..` in rust only takes fields (move) not mentioned.
  ```rust
  struct Point {
    x: i32,
    y: i32,
  }
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 15, ..p1 }; // p2 = { 15, 20 }
  ```
- tuple-like struct is good for _newtypes_ since you get stricter type checking.
  ```rust
  struct Ascii(Vec<u8>);
  ```
- values of unit-like struct occupy no memory and no generated code.
- struct fields' values might be not stored in the order they are in struct (you can specify layout like `#[repr(C)]`).
- a method's `self` argument can also be a `Box<Self>`, `Rc<Self>`, or `Arc<Self>` (same for `&self` and `&mut self`).
  ```rust
  struct Queue {}
  impl Queue {
    pub fn push(&mut self, c: char) {}
  }
  let mut q = Queue {};
  q.push('h');
  let mut bq = Box::new(Queue {});
  bq.push('e');
  ```
- there are associated functions and consts.
  ```rust
  impl Queue {
    const ZERO: Queue = Queue {};
    const NAME: &str = "Queue";
    pub fn new() -> Queue {
      Queue {}
    }
  }
  ```
- generic struct is similar to c++ template, so you can specialize if needed.
- when you need mutable data inside an immutable value, there are `Cell<T>` and `RefCell<T>`.

### 10. Enums and Patterns

- enum is similar to Haskell's algebraic data types.
- enum without data is similar to C enum (default and following values). You can cast enum to integer, but integer to enum is not allowed.
  ```rust
  enum Status {
    Created,
    Pending = 10,
    Completed,
  }
  println!("{}", Status::Completed as i32); // 11
  ```
- there are 3 kinds of enum variants, echoing 3 kinds of struct. All constructions and fields share the same visibility of the enum.
  ```rust
  enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    },
  }
  ```
- enum with data is stored as a small integer tag, plus enough memory to hold all the fields the largest variant.
  ![enum](https://i.imgur.com/O0kGJKX.png)
- there are special cases, Rust can eliminate the tag field. For example, `Option<T>` when T is a reference (`Box` or smart pointer types), since T is cannot be null, so `None` can be represented as 0, `Some` for pointers.
- pattern matching supports various types: literal, variable, tuple, struct, array, reference ...
  ```rust
  match get_account(id) {
    Some(Account { name, language, ..}) => something // match name, language and ignore other fields
  }
  ```
- matching a non copyable value moves the value.
  ```rust
  match account {
    Account { name, language, .. } => {
        ui.greet(&name, &language);
        ui.show_settings(&account);  // error: borrow of moved value: `account`
    }
  }
  // borrow instead
  match account {
    Account { ref name, ref language, .. } => {
        ui.greet(name, language);
        ui.show_settings(&account);  // ok
    }
  }
  ```
- there are two kind of patterns:
  - irrefutable pattern always matches, `let` and `for` only accept this pattern.
  - refutable pattern might not match.

### 11. Traits and Generics

- traits is inspired by Haskell's typeclass, while generics is similar to C++ template (generate machine code for each type `T` that you use).
- to use traits methods for a type, you have to explicitly import that traits.
- C# interface, a value of type T is a reference to any object that implements T, the same for Rust.
  ```rust
  use std::io::Write;
  let mut buf: Vec<u8> = vec![];
  let writer: dyn Write = buf;  // error: `Write` does not have a constant size
  let writer: &dyn Write = &buf;  // ok
  let writer: &mut dyn Write = &mut buf;  // ok, creating fat pointer (trait object) contains a pointer to data and a pointer to vtable
  ```
  [trait object](https://i.imgur.com/RSudX7c.png)
- unlike C++, the vtable pointer is stored as part of the struct, Rust uses flat pointers so a struct can implement dozens of traits without containing dozens of vtable pointers -> a method call will be dynamic dispatching.
- references and smart pointers (`Box`, `Rc` ...) are converted to trait objects when needed.
- difference between generic and trait
  - generic generates machine code for each type you use (easy to optimize and better speed).
  - trait object uses dynamic dispatch.
  ```rust
  fn say_hello(out: &mut dyn Write)     // plain
  fn say_hello<W: Write>(out: &mut W)   // generic
  fn say_hello(out: &mut dyn Write)     // trait
  ```
- everything defined in a trait `impl` must be a feature of trait (if you need a helper method, defining in `impl` of that type).
  ```rust
  trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
  }
  impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }
    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
        && self.y - self.height - 1 <= y
        && y <= self.y
    }
  }
  impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
  }
  ```
- when implementing a trait, either the trait or the type must be local in the current crate (_orphan rule_) to ensure that other people's code cannot break yours and vice versa.
- a trait that uses `Self` is incompatible with trait objects
  ```rust
  pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self; // requires self and other are the same type
  }
  impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
    }
  }
  impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self {
    }
  }
  fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable) {
    let combo = left.splice(right); // error since Rust doesn't know at compile type if `left and `right` are the same type as required.
  }
  // if we replace with the trait below, it will works.
  pub trait MegaSpliceable {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
  }
  ```
- trait can include type-associated functions. If you want to use `&dyn StringSet` (trait object), you have to add the bounding and those bounding functions are excluded.
  ```rust
  trait StringSet {
    fn new() -> Self;
    fn contains(&self, string: &str) -> bool;
    ----
    fn new() -> Self          // for trait object usage since it is not included -> only can use contains
        where Self: Sized;
  }
  ```
- associated types trait defines one specific related type for each implementation.
  ```rust
  pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
  }
  impl Iterator for Args {
    type Item = String;
    fn next(&mut self) -> Option<String> {
    }
  }
  ```
- generic type get a special dispensation, you can implement a foreign trait for a foreign type, as long as one of the trait's type parameters is defined in the local crate.
  ```rust
  struct Meters(u32);
  impl Add<Meters> for Millimeters {            // Milimeters is the 3rd crate
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }
  ```
- associated trait consts can be declared without giving a value, then implementors of that trait can define those.
  ```rust
  trait Greet {
    const GREETING: &'static str = "Hello";
    const ZERO: Self;
  }
  impl Greet for f32 {
    const ZERO: f32 = 0.0;
  }
  ```

### 12. Operator Overloading

- arithmetic expressions are shorthand for method calls like `a + b` -> `a.add(b)` (`add` belongs `std::ops::Add`).
  ```rust
  trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
  }
  assert_eq!(10.add(20), 10 + 20); // ok
  ```
- `std::cmp::PartialEq` contains 2 methods. Since `ne` method has a default definition, you only need to define `eq`.
  ```rust
  trait PartialEq<Rhs = Self>
  where
      Rhs: ?Sized,
  {
      fn eq(&self, other: &Rhs) -> bool;
      fn ne(&self, other: &Rhs) -> bool {
          !self.eq(other)
      }
  }
  ```
- math equivalence relation imposes three requirements below. `PartialEq` is used for the `==` operator's built-in trait because `==` doesn't meet the third one like `NaN` is not equal to `NaN`. There is `Eq` trait that represents all.
  - `x == y` -> `y == x`.
  - `x == y` and `y == z` -> `x == z`.
  - `x == x`.
- among all primitive types, only comparisons between floating-point values return `None` (`NaN` with anything else returns `None`).
- `std::cmp::PartialOrd` contains 5 methods but 4 methods have default definition. The only you have to implement is `partial_cmp`.
  ```rust
  trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
  where
      Rhs: ?Sized,
  {
      fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>; // None means self and other are unordered with each other (like NaN)
      fn lt(&self, other: &Rhs) -> bool { ... }
      fn le(&self, other: &Rhs) -> bool { ... }
      fn gt(&self, other: &Rhs) -> bool { ... }
      fn ge(&self, other: &Rhs) -> bool { ... }
  }
  ----
  enum Ordering {
    Less,       // self < other
    Equal,      // self == other
    Greater,    // self > other
  }
  ```
- depends on the expression (like borrowing), Rust translates to either `std::ops::Index` or `std::ops::IndexMut`.
  ```rust
  let mut m = HashMap::new();
  assert_eq!(m["十"], 10);
  assert_eq!(m["千"], 1000);
  // ->
  assert_eq!(*m.index("十"), 10);
  assert_eq!(*m.index("千"), 1000);
  ```
  ```rust
  let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
  desserts[0].push_str(" (fictional)");
  desserts[1].push_str(" (real)");
  // ->
  (*desserts.index_mut(0)).push_str(" (fictional)");
  (*desserts.index_mut(1)).push_str(" (real)");
  ```
- not all operators can be overloaded like `?`, `=`, `&&`, `||`, function call operator ...

### 13. Utility Traits

- There are 3 categories of traits
  - Language extension traits: integrate your own types closely to the language like `Drop`, `Deref` ...
  - Marker traits: express constaints to bound generic type variables like `Sized` and `Copy`.
  - Public vocabulary traits like `Default`, `Borrow` ...
- `Drop` trait which is analogous to C++ destructor is called (dropping in stack, heap and system resources) when a value's owner goes away (out of scope, end of expression statement, truncate a vector ...). Usually, it is handled automatically but you can define it needed.
  ```rust
  struct Appellation {
    name: String,
    nicknames: Vec<String>
  }
  impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
  }
  ```
- `Copy` trait is for types whose values can be duplicated simply by copying bits (on stack) while `Clone` trait is always explicit and may or may not expensive.
- `std::ops::Deref/DerefMut` traits specify how deferencing operators `*` and `.` behave. They are designed for implementating smart pointers like `Box`, `Rc`, `Arc` ...
  ```rust
  trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
  }
  trait DerefMut: Deref {
      fn deref_mut(&mut self) -> &mut Self::Target;
  }
  ```
- Since `deref` takes `&Self` reference and returns a `&Self::Target` reference, references can be converted to the latter. Rust will apply several deref coercions in succession if necessary.
  ```rust
  struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,
    /// The index of the "current" element in `elements`. A `Selector`
    /// behaves like a pointer to the current element.
    current: usize
  }
  impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
  }
  let mut s = Selector { elements: vec!['x', 'y', 'z'],
                       current: 2 };
  // Because `Selector` implements `Deref`, we can use the `*` operator to
  // refer to its current element.
  assert_eq!(*s, 'z');
  // Assert that 'z' is alphabetic, using a method of `char` directly on a
  // `Selector`, via deref coercion.
  assert!(s.is_alphabetic());
  // Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
  *s = 'w';
  // ---------
  assert_eq!(s.elements, ['x', 'y', 'w']);
  let s = Selector { elements: vec!["good", "bad", "ugly"],
                   current: 2 };
  fn show_it(thing: &str) { println!("{}", thing); }
  show_it(&s);
  // type of &s is &Selector<&str>, due to Deref<Target=str> -> show_it(s.deref())
  // if you change show_it to generic function, it doesn't work since Rust doesn't try deref coercions to satisfy type variable bounds
  ```
