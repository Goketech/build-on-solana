fn concatenate_strings(a: &String, b: &String) -> String {
    let mut result = a.clone();
    result.push_str(b);
    result
}

fn main() {
    let string1 = String::from("Hello, world!");
    let string2 = String::from("Life is good!");
    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("concatenated_string: {}",concatenated_string);
}