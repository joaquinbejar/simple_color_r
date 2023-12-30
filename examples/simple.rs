use simple_color::utils::give_color;
use simple_color::colors::{Colors};
fn main() {
    println!("{}", give_color(&Colors::Red, "Hello, world!", false));
}
