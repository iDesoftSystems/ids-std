/// capitalize words
pub fn capitalize(value: &str) -> String {
    let mut result = String::new();
    let mut first = true;

    for value in value.trim().to_lowercase().chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
            if value == ' ' {
                first = true;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::capitalizer::capitalize;

    #[test]
    fn it_capitalize_lowercase_words() {
        let to_capitalize = "your ideas, your world";
        let expected = "Your Ideas, Your World";

        let result = capitalize(to_capitalize);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_capitalize_uppercase_words() {
        let to_capitalize = "YoUR iDeas, yOuR WoRlD";
        let expected = "Your Ideas, Your World";

        let result = capitalize(to_capitalize);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_trim_spaces() {
        let to_capitalize = " YoUR iDeas, yOuR WoRlD ";
        let expected = "Your Ideas, Your World";

        let result = capitalize(to_capitalize);
        assert_eq!(result, expected)
    }
}
