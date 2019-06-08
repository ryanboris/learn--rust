pub fn run() {
    // Print to console
    println!("Hello from the print rs file.");

    // Basic formatting
    println!("Number: {} and {} and {}", 1, 2, 3);

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to code.", "Brad", "Mass");

    // Named Arguments
    println!(
        "{name} likes to play {activity}!",
        name = "John",
        activity = "soccer"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math

    println!("10 + 10 = {}", "10 + 10");
}
