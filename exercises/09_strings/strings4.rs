// Calls of this function should be replaced with calls of `string_slice` or `string`.
enum Input {
    String(String),
    Str(&'static str),
}
fn placeholder(input: Input) {
    match input {
        Input::String(s) => string(s),
        Input::Str(s) => string_slice(s),
    }
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    placeholder(Input::Str("blue"));

    placeholder(Input::String("red".to_string()));

    placeholder(Input::String(String::from("hi")));

    placeholder(Input::String("rust is fun!".to_owned()));

    placeholder(Input::Str("nice weather".into()));

    placeholder(Input::String(format!("Interpolation {}", "Station")));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    placeholder(Input::Str(&"abc"[0..1]));

    placeholder(Input::Str("  hello there ".trim()));

    placeholder(Input::String("Happy Monday!".replace("Mon", "Tues")));

    placeholder(Input::String("mY sHiFt KeY iS sTiCkY".to_lowercase()));
}
