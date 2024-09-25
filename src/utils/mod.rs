// get_port returns the port number from the command line arguments. If no 
// port number is provided, it returns the default port number 3000.
pub fn get_port(args: &Vec<String>) -> String {
    return match args.get(1) {
        Some(port) => port.clone(),
        None => "3000".to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_to_strings(input: &str) -> Vec<String> {
        input.split(" ").map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_get_port() {
        assert_eq!(get_port(&input_to_strings("test 4001 third")), "4001");
        assert_eq!(get_port(&input_to_strings("test 4000")), "4000");
        assert_eq!(get_port(&input_to_strings("test")), "3000");
    }
}