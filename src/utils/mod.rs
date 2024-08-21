pub fn get_port(args: &Vec<String>) -> String {
    return match args.get(1) {
        Some(port) => port.clone(),
        None => "3000".to_string(),
    };
}