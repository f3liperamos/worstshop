use std::fs;

pub fn read_file(filename: &str) -> String {
    let file = fs::read_to_string(filename).expect("file not found");
    file
}

#[cfg(test)]
mod test {
    use super::*; // imports outer scope into test scope

    #[test]
    fn it_reads_to_string() {
        assert_eq!(
            read_file("src/input_test"),
            "Roses are Red\nViolets are Blue\n\nUnexpected '{'\non line 32.\n"
        );
    }
}
