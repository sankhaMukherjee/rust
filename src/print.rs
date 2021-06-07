pub fn run() {
    println!("Hello from the print.rs file ...");

    println!("{} is form {} and {} likes to {}", "part1", "part2", 3, 12.4);

    println!("{0} is form {1} and {1} likes to {0}", "part2", 12.4);

    println!(
        "{name} is form {city}",
        name = "Sankha", city = "Singapore"
    );

    println!("{:?}", (12, true, "holla"));
}