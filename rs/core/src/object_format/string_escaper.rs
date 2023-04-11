pub fn escape_string(input: &str, escape_chars: &[char]) -> String {
    input
        .chars()
        .into_iter()
        .map(
            |input_char| match escape_chars.contains(&input_char) || input_char == '\\' {
                true => format!("\\{}", input_char),
                false => format!("{}", input_char),
            },
        )
        .collect::<Vec<String>>()
        .join("")
}
