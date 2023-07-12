// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope

// run on command line
// rustc split.rs && ./split
mod my;

fn function() {
    println!("spilt.rs :: called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
