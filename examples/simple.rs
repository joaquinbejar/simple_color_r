use simple_color::colors::Colors;
use simple_color::utils::give_color;
fn main() {
    println!("{}", give_color(&Colors::Red, "Hello, world!", false));
}
