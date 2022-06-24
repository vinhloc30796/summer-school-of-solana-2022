/*
fn function_name() -> return_type {
    return value;
}

fn function_name() -> return_type {
    value
}
*/
#[allow(dead_code)]
pub fn fn_hello() -> &'static str {
    "Hello from Barcelona"
}

#[allow(dead_code)]
pub fn fn_hello_extended(place: Option<String>) -> String {
    let prefix = "Hello from ";
    prefix.to_owned() + &place.unwrap_or("Vietnam".to_string())
}
