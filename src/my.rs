// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my(my.rs)::function()`");
}

fn private_function() {
    println!("private_function:: called `my(my.rs)::private_function()`");
}

pub fn indirect_access() {
    print!("called `my(my.rs)::indirect_access()`, that\n> ");

    private_function();
}