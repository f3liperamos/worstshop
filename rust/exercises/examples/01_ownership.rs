fn join_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

fn invert_name(first_name: String, last_name: String) -> String {
    format!("{} {}", last_name, first_name)
}

fn main() {
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let opposite = invert_name(first_name, last_name);
    let full_name = join_name(first_name, last_name);

    // TODO: fix me
    assert_eq!(full_name, "John Doe");
}
