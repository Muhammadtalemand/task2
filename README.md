# Second task
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`task2 = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "task2"
version = "0.1.0"
authors = ["muhammadtalemand <talemandkhokhar@gmail.com>"]
edition = "2018"

[dependencies]
task1 = "0.1.0"
```
In `src/lib.rs` you can use like this:

```
pub mod Hotel{
    pub mod floors{
        pub fn room(){
            println!("Please  access to the Room 5 at floor No. 4.");
        }
    }
}

```

In `src/main.rs` you can use like this:

```
//making module of lib.rs file
mod lib;
//using use keyward to access the lib.rs file
use lib::Hotel::floors;


fn main() {
    crate::lib::Hotel::floors::room();// absolute path
    //Use keyward
    floors::room();
}
```

now `cargo run` for results
