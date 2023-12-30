


#[cfg(test)]
mod tests {
    use simple_color::colors::{BLACK, RED, YELLOW, Colors};
    use simple_color::utils::give_color as give_color;


    #[test]
    fn test_color_constants() {
        assert_eq!(BLACK, "\\x1b[38;5;000m");
        assert_eq!(RED, "\\x1b[38;5;009m");
        assert_eq!(YELLOW, "\\x1b[38;5;011m");
    }

    #[test]
    fn test_give_color() {
        let text = "Hello, world!";
        let colored_text = give_color(&Colors::Red, text, false);
        assert_eq!(colored_text, format!("{}{}\\033[0m", RED, text));

        let blinking_text = give_color(&Colors::Red, text, true);
        assert_eq!(blinking_text, format!("\\033[5m{}{}\\033[0m", RED, text));
    }
}
