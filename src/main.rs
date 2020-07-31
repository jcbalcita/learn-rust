fn main() {
    // pattern matching assignment
    let (mut bunnies, carrots) = (8, 50);
    bunnies = 3;
    // carrots = 5   <= causes error

    // constants (must be definable at COMPILE time)
    const THIS_CONSTANT: &str = "that";

    hello_world();
}

fn hello_world() -> &str {
    const HELLO_WORLD: &str = "Hello, world!";
    println!(HELLO_WORLD);
    HELLO_WORLD
}

