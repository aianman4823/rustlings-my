// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color(s:&str) -> &str {
    s
}

fn main() {
    let favorite_color = "blue";
    let answer = current_favorite_color(favorite_color);
    println!("My current favorite color is {answer}");
}
