---
title: TechCamp 2024/10 Rust Web Frameworks
author: Selim Kälin & Zak Cook & Jan Kleine
institute: Innovation Process Technology AG
date: 12.10.2024
theme: moon
revealjs-url: "https://unpkg.com/reveal.js@5.1.0"
progress: false
controls: false
hash: true
highlightjs: true
---

# Actix

## Ergonomics / Hands-On Feel

## Benchmark

# Axum

## Ergonomics / Hands-On Feel

## Benchmark

# Rocket

## Ergonomics {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|1,4" rust>
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
}
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|5-7,12" rust>
#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, mongodb};

#[derive(Database)]
#[database("my-database-name")]
pub struct MyDatabase(mongodb::Client);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyDatabase::init())
}
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|12|12-24|30|2-6" rust><script type="text/template">
#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, Connection, mongodb};
use rocket::serde::uuid::Uuid;
use rocket::http::Status;
use rocket::serde::json::{json, Value};

#[derive(Database)]
#[database("my-database-name")]
pub struct MyDatabase(mongodb::Client);

#[get("/texts/<uuid>")]
pub async fn get(db: Connection<MyDatabase>, uuid: Uuid) -> (Status, Value) {
    match get_from_database(db, uuid).await {
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("error searching database: {e}")}),
        ),
        Ok(result) => (
            Status::Ok, 
            json!({"data": text.text.to_owned()}),
        ),
    },
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyDatabase::init())
        .mount("/", routes![get])
}
</script></code></pre>

## Benchmark

Will update this when available ...

## Immutability {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="2,4" rust>
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
</code></pre>

::: notes
- typing is still respected, cannot change type of variable, even with `mut`
:::

## Everything is Owned

```{.rust data-line-numbers=""}
let x = String::from("hello");
let y = x;
println!("{}", x); // invalid
```

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  fn print_hello(name: String) {
      println!("Hello, {}!", name);
  }
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  fn print_hello(name: String) {
      println!("Hello, {}!", name);
  }

  fn main() {
      let name = String::from("Jakob");
      print_hello(name);
      // can no longer use name
  }
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  fn print_hello(name: String) {
      println!("Hello, {}!", name);
  }

  fn main() {
      let name = String::from("Jakob");
      print_hello(name.clone());
      print_hello(name);
  }
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  fn print_hello(name: &str) {
      println!("Hello, {}!", name);
  }

  fn main() {
      let name = String::from("Jakob");
      print_hello(&name);
      print_hello(&name);
  }
</code></pre>

## Mutable Move

```{.rust data-line-numbers=true}
fn print_hello(mut name: String) {
    name.push_str(" Beckmann");
    println!("Hello, {}!", name);
}
```
## Lifetimes

```{.rust data-line-numbers="|2-11|4-6|8-10"}
fn main() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }
}
```

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|4" rust>
  struct A { x: i32 }

  impl A {
      fn borrow(&self) -> &i32 {
          &self.x
      }
  }

  fn main() {
      let a = A { x: 1 };
      println!("{}", a.borrow());
  }
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="4|" rust>
  struct A { x: i32 }

  impl<'a> A {
      fn borrow(&'a self) -> &'a i32 {
          &self.x
      }
  }

  fn main() {
      let a = A { x: 1 };
      println!("{}", a.borrow());
  }
</code></pre>
::: notes
- lifetime parameter treated as generic definition
- inferred with only one argument and return
- needed with more than one argument
:::

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  struct A<'a> { x: &'a i32 }

  impl<'a> A<'a> {
      fn borrow(&'a self) -> &'a i32 {
          self.x
      }
  }

  fn main() {
      let x = 2;
      let a = A { x: &x };
      println!("{}", a.borrow());
  }
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|13-15" rust>
  struct A<'a> { x: &'a i32 }

  impl<'a> A<'a> {
      fn borrow(&'a self) -> &'a i32 {
          self.x
      }
  }

  fn main() {
      let x = 2;
      let a = A { x: &x };
      {
          let y = 3;
          a.x = &y;
      }
      println!("{}", a.borrow());
  }
</code></pre>

##

```text
error[E0597]: `y` does not live long enough
  --> src/main.rs:14:15
   |
13 |         let y = 3;
   |             - binding `y` declared here
14 |         a.x = &y;
   |               ^^ borrowed value does not live long enough
15 |     }
   |     - `y` dropped here while still borrowed
16 |     println!("{}", a.borrow());
   |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
```

::: notes
Ending: let's get back to the code ...
:::

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  struct A<'a> { x: &'a i32 }

  impl<'a> A<'a> {
      fn borrow(&'a self) -> &'a i32 {
          self.x
      }
  }

  fn main() {
      let x = 2;
      let a = A { x: &x };
      {
          let y = 3;
          a.x = &y;
      }
      println!("{}", a.borrow());
  }
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  struct A<'a> { x: &'a i32 }

  impl<'a> A<'a> {
      fn borrow(&'a self) -> &'a i32 {
          self.x
      }
  }

  fn main() {
      let x = 2;
      let a = A { x: &x };
      {
          let y = 3;
          a.x = &y;
      }
  }
</code></pre>

::: notes
Compiler is smart enough to figure out we don't access the reference after the lifetime
:::


# Hands-On

Rustlings:

- Immutability:
  - `exercises/01_variables`, try 4 and 5
- Ownership:
  - `exercises/06_move_semantics`, try 2 to 5
- Lifetimes:
  - `exercises/16_lifetimes`, try 1 to 3

::: notes
30 minutes
:::

# Enums

##

```{.rust data-line-numbers=true}
// Simple enum 
enum Seasons {
    Winter,
    Spring,
    Summer,
    Autumn,
    Bulk,
}

let favorite_season = Seasons::Winter;
let mut current_season = Seasons::Summer;
```

::: notes
- enums are not used to group related fields (like structs)
- enums give a way of saying that a value is one of a set of possibilities
:::

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
  // Enums can contain data
  enum Contact{
    Phone(u16, u16, u16, u16),
    Email(String),
  }

  let ipt_phone = Contact::Phone(44, 735, 27, 69);
  let ipt_email = Contact::Email(String::from("info@ipt.ch"));
</code></pre>

::: notes
useful to concisely represent data instead of including an enum inside a struct or similar
constructor is automatically generated for enum containing fields
can contain any kind of data, even structs or other enums
:::

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|10" rust>
  // Enums can contain data
  enum Contact{
    Phone(u16, u16, u16, u16),
    Email(String),
  }

  let ipt_phone = Contact::Phone(44, 735, 27, 69);
  let ipt_email = Contact::Email(String::from("info@ipt.ch"));

  ipt_phone.contact();
</code></pre>

## 

```text
error[E0599]: no method named `contact` found for enum `Contact` in the current scope
  --> src/main.rs:13:13
   |
2  |   enum Contact{
   |   ------------ method `contact` not found for this enum
...
13 |   ipt_phone.contact();
   |             ^^^^^^^ method not found in `Contact`
```

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|7-15" rust>
  // Enums can contain data
  enum Contact{
    Phone(u16, u16, u16, u16),
    Email(String),
  }

  // Enums can implement methods
  impl Contact{
    fn contact(&self) {
      match self {
        Contact::Phone(a, b, c, d) => println!("dialling {}-{}-{}-{} ...", a, b, c, d),
        Contact::Email(address) => println!("writing to {} ...", address),
      }
    }
  }

  let ipt_phone = Contact::Phone(44, 735, 27, 69);
  let ipt_email = Contact::Email(String::from("info@ipt.ch"));

  ipt_phone.contact(); 
</code></pre>

::: notes
methods can be defined on enums
self will be the value that the method gets called on, e.g.
self = ipt_phone = Contact::Phone(...) above
:::

##

```text
dialling 44-735-27-69 ...
```

## Rust's `Option` Enum 

```{.rust data-line-numbers=true}
enum Option<T> {
    None,
    Some(T),
}

let apples: Option<u8> = Some(4);
let orange: Option<u8> = None;
```

::: notes
value inside Some(T) must be of type defined with Option<T>
:::

## Rust's `Result` Enum 

```{.rust data-line-numbers=true}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

::: notes
Ok type T can (and usually is) differnt from Err type E
Results must be used -> compiler will warn of unused/ignored Result values
both Ok and Err variants can be () -> but does it make sense?
:::

# Pattern Matching

##

```{.rust data-line-numbers=true}
fn real_season(season: Seasons) -> Result<Seasons, String> {
     match season {
          Seasons::Winter => Ok(season),
          Seasons::Spring => Ok(season),
          Seasons::Summer => Ok(season),
          Seasons::Autumn => Ok(season),
          Seasons::Bulk => Err("I am not so sure about that...".to_string()),
     }
}
```

## 
```{.rust data-line-numbers=true}
  fn real_season(season: Seasons) -> Result<Seasons, String> {
      match season {
          other => Ok(other),
          Seasons::Bulk => Err("I am not so sure about that...".to_string()),
      }
  }
```

## 
```{.rust data-line-numbers=true}
  // Remember our apples and oranges
  fn buy_more(fruit: Option<u8>) -> bool {
    match fruit {
      None => true,
      Some(x) => false,
    }
  }
```

::: notes
matches are exhaustive
can use wildcard catch-all pattern by defining a variable that will be used for all non-covered cases
can use _ to do something with non-covered cases without reusing the value
:::

# Error Handling

## To Panic or Not Panic 

```{.rust data-line-numbers=true}
// Unrecoverable errors use panic! macro
if totally_broken {
    panic!("nothing we can do about this");
}
```

---

```{.rust data-line-numbers=true}
// Recoverable errors using Result
let some_variable = function_that_could_fail();
match some_variable {
     Ok(result) => use_result(result),
     Err(error) => println!("this error occurred: {error:?}"),
}

// Shortcuts for Result type
let only_valid = function_that_could_fail().unwrap();
let only_valid = function_that_could_fail()
    .expect("oh no something bad happened");

// Or even shorter (only if return types align)
let only_valid = function_that_could_fail()?;
```

::: notes
when should you panic?
-> there is no way back, program crashes
when to use result type and unwrap?
-> calling code has option to deal with error gracefully
:::

## Match on Different Errors

```rust
  use std::fs::File;

  fn read_tweet(source_path: &str, buffer: &mut [u8]) -> usize {
    let tweet = File::open(source_path);

    match tweet {
      Ok(file) => {
        let bytes_read = file.read(&mut buffer);
        match bytes_read {
          Ok(number_of_bytes) => number_of_bytes,
          Err(err) => panic!("Failed to read the tweet: {err:?}"),
        },
      },
      Err(err) => panic!("Failed to open the tweet: {err:?}"),
    }
  }
```

::: notes
all three topics, 20 minutes
unwrap_or_else() -> closure that allows a potential error to be used for further processing
:::


# Hands-On

Rustlings:

- Enumerations -> `exercises/08_enums`, try 1 and 2
- Pattern Matching -> `exercises/12_options`, try 2 and 3
- Error Handling -> `exercises/13_error_handling`, start with 1

::: notes
20 minutes
:::

# Traits

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
// everyone has seen interfaces
trait Closer {
  fn close(self) -> Result<(), &'static str>;
}
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
// everyone has seen interfaces
trait Closer {
  fn close(self) -> Result<(), &'static str>;
}

struct File {
    // fields...
}

impl Closer for File {
    fn close(self) -> Result<(), &'static str> {
        Err("not implemented")
    }
}
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
// everyone has seen interfaces
trait Closer {
  fn close(self) -> Result<(), &'static str>;
}

// you can implement traits for any type
impl Closer for String {
    fn close(mut self) -> Result<(), &'static str> {
        self.clear();
        Ok(())
    }
}
</code></pre>


## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
// everyone has seen interfaces
trait Closer {
  fn close(self) -> Result<(), &'static str>;
}

// restrict your functions to take types implementing a trait
fn close_and_exit(c: impl Closer) {
    let _ = c.close();
    // exit
}
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers="9" rust>
// everyone has seen interfaces
trait Closer {
  fn close(self) -> Result<(), &'static str>;
}

// restrict your functions to take types implementing a trait
fn close_and_exit(c: impl Closer) {
    let _ = c.close();
    let _ = c.close(); // what would happen here?
    // exit
}
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
trait Closer {
  // self is moved
  fn close(self) -> Result<(), &'static str>;
}

trait TextStream {
  // mutable borrow of self
  fn read(&mut self) -> String;
  // immutable borrow of self
  fn at_end(&self) -> bool;
}

fn read_until_end(mut io: impl Closer + TextStream) {
    while !io.at_end() {
        let _ = io.read();
    }
    io.close();
}
</code></pre>

## Built-in traits
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
use std::fmt::Debug;

struct Point {
    x: u32,
    y: u32
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "x: {:?}, y: {:?}", self.x, self.y)
    }
}

fn main() {
    let origin = Point{x: 0, y: 0};
    println!("the origin is at {:?}", &origin);
}
</code></pre>

# Macros

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
// define a new macro with "macro_rules!"
macro_rules! say_hello {
    // the macro takes no argument
    () => {
        // the macro will expand into the contents of this block
        println!("Hello!")
    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!();
}
</code></pre>

## {data-auto-animate=true}
<pre data-id="code-animation"><code data-trim data-line-numbers rust>
// define a new macro with "macro_rules!"
macro_rules! say_hello {
    // the macro takes no argument
    () => {
        // the macro will expand into the contents of this block
        println!("Hello!")
    };
    ($expression:expr) => {
        println!("Hello, {}!", $expression)
    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!();
    // This call will expand into `println!("Hello, {}!", "world")`
    say_hello!("world");
}
</code></pre>

## the `vec!` macro

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
macro_rules! vec {
    () => { ... };
    ($elem:expr; $n:expr) => { ... };
    // variadic argument support (dunno wtf this is)
    ($($x:expr),+ $(,)?) => { ... };
}
</code></pre>

## `derive` macros

<pre data-id="code-animation"><code data-trim data-line-numbers rust>
use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

fn main() {
    let origin = Point{x: 0, y: 0};
    println!("the origin is at {:?}", &origin);
}
</code></pre>

::: notes
20 minutes
:::


# Hands-On

- `exercises/15_traits/` 1, 2, 4, 5
- `exercises/21_macros/` 1, 2, 4

::: notes
20 minutes
:::
