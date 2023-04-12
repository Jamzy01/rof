export function escape_string(input: string, escape_chars: string[]): string {
    return Array.from(input).map(input_char => {
        if (escape_chars.includes(input_char)) {
            return "\\" + input_char;
        }

        else {
            return input_char;
        }
    }).join("")
}