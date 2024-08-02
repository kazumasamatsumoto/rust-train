fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("Base 16 (hexadecimal): {:X}", 69420);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>5}", number=1);

    println!("{number:0>5}", number=1);

    println!("{number:0<5}", number=1);

    println!("{number:0>width$}", number=1, width=5);

    println!("My name is {0}, {1} {0}", "Bond", "James");
}
