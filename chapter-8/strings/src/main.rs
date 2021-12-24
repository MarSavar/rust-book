fn main() {
    let hello = String::from("こんにちは, ");
    let name = String::from("マリオ");

    // Deref coercion below. name is turned from a String to a &str type because
    // of how + works. The signature of add takes a string slice as a param.
    // Ownership of hello is lost after assigning its value to greeting, so
    // hello can no longer be used after the following operation.
    let greeting = hello + &name;
    println!("{}", greeting);

    let name_literal = "マリオ"; // type is &str, not String
    let good_evening = String::from("こんばんは, ");
    println!("{}", good_evening + name_literal); // no need to reference &, literal is a slice

}
