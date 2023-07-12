pub fn function() {
    println!("called `my::nested(nested.rs)::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested(nested.rs)::private_function()`");
}