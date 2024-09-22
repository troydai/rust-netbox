// get_port returns the port number from the command line arguments. If no 
// port number is provided, it returns the default port number 3000.
pub fn get_port(args: &Vec<String>) -> String {
    return match args.get(1) {
        Some(port) => port.clone(),
        None => "3000".to_string(),
    };
}
