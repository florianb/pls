fn main() {
    let args: Vec<String> = std::env::args().collect();
    let first_arg = args.get(1).expect("argument missing");

    println!("{first_arg}");
    std::process::Command::new(&*first_arg)
        .status()
        .expect("Oops");
}
