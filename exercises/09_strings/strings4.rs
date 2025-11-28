// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    // &str literal
    string_slice("blue");

    // String created via to_string()
    string("red".to_string());

    // String created via String::from
    string(String::from("hi"));

    // String created via to_owned()
    string("rust is fun!".to_owned());

    // String created via into()
    string("nice weather".into());

    // String created via format!
    string(format!("Interpolation {}", "Station"));

    // &str slice of a String
    string_slice(&String::from("abc")[0..1]);

    // &str returned by trim()
    string_slice("  hello there ".trim());

    // String returned by replace()
    string("Happy Monday!".replace("Mon", "Tues"));

    // String returned by to_lowercase()
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
