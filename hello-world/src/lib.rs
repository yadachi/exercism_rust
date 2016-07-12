pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(r) => format!("Hello, {}!", r),
        None => "Hello, World!".to_string()   
    }
}
