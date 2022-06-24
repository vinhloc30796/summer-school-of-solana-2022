#[allow(dead_code)]
pub fn print_string_obj() {
    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    // let content_string = String::from("AckeeBlockchain");
    let content_string: &str = "AckeeBlockchain";
    println!("length of {} is {}", content_string, content_string.len());
}
