fn main() {
    println!("What is your name?");
    let mut demo_name = "Dylan";

    let demo_bool: bool = true;
    if demo_bool {
        println!("Demo bool is true");
        print!("Hello {demo_name}");
    }

    demo_name = "Dylan Number 2";

    println!("New demo name is {demo_name}");

    println!("Min of i64 is {}", std::i64::MIN);
    println!("Max of i64 is {}", std::i64::MAX);

    println!("Max of u128 is {}", std::u128::MAX);
}
