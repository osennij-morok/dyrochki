use phf::phf_map;

use crate::i18n;

#[cfg(test)]
mod tests;

pub fn count_to_stdout(text: &str) {
    let counting_result: HolesCountingResult = count(&text);
    println!("{}", i18n::holes::holes_found_msg(counting_result.holes_count));
    if counting_result.uncounted_chars.len() > 0 {
        let uncounted_chars_str: String = i18n::holes::uncounted_chars_msg(&counting_result.uncounted_chars);
        println!("{}", uncounted_chars_str);
    }
}

pub fn count(text: &str) -> HolesCountingResult {
    let mut total_holes_count: usize = 0;
    let mut uncounted_chars: Vec<char> = Vec::new();
    for ch in text.chars() {
        if ch.is_whitespace() {
            continue;
        }
        match HOLES_MAP.get(&ch) {
            Some(holes_in_char) => total_holes_count += holes_in_char,
            None => uncounted_chars.push(ch)
        }
    }
    uncounted_chars.dedup();
    return HolesCountingResult {
        holes_count: total_holes_count, 
        uncounted_chars: uncounted_chars.clone() 
    };
}

#[derive(Debug)]
pub struct HolesCountingResult {
    pub holes_count: usize,
    pub uncounted_chars: Vec<char>,
}

static HOLES_MAP: phf::Map<char, usize> = phf_map! {
    'A' => 1, 'a' => 1,
    'B' => 2, 'b' => 1,
    'C' => 0, 'c' => 0,
    'D' => 1, 'd' => 1,
    'E' => 0, 'e' => 1,
    'F' => 0, 'f' => 0,
    'G' => 0, 'g' => 1,
    'H' => 0, 'h' => 0,
    'I' => 0, 'i' => 0,
    'J' => 0, 'j' => 0,
    'K' => 0, 'k' => 0,
    'L' => 0, 'l' => 0,
    'M' => 0, 'm' => 0,
    'N' => 0, 'n' => 0,
    'O' => 1, 'o' => 1,
    'P' => 1, 'p' => 1,
    'Q' => 1, 'q' => 1,
    'R' => 1, 'r' => 0,
    'S' => 0, 's' => 0,
    'T' => 0, 't' => 0,
    'U' => 0, 'u' => 0,
    'V' => 0, 'v' => 0,
    'W' => 0, 'w' => 0,
    'X' => 0, 'x' => 0,
    'Y' => 0, 'y' => 0,
    'Z' => 0, 'z' => 0,

    '0' => 1,
    '1' => 0,
    '2' => 0,
    '3' => 0,
    '4' => 0,
    '5' => 0,
    '6' => 1,
    '7' => 0,
    '8' => 2,
    '9' => 1,

    'А' => 1, 'а' => 1,
    'Б' => 1, 'б' => 1,
    'В' => 2, 'в' => 2,
    'Г' => 0, 'г' => 0,
    'Д' => 1, 'д' => 1,
    'Е' => 0, 'е' => 1,
    'Ё' => 0, 'ё' => 1,
    'Ж' => 0, 'ж' => 0,
    'З' => 0, 'з' => 0,
    'И' => 0, 'и' => 0,
    'Й' => 0, 'й' => 0,
    'К' => 0, 'к' => 0,
    'Л' => 0, 'л' => 0,
    'М' => 0, 'м' => 0,
    'Н' => 0, 'н' => 0,
    'О' => 1, 'о' => 1,
    'П' => 0, 'п' => 0,
    'Р' => 1, 'р' => 1,
    'С' => 0, 'с' => 0,
    'Т' => 0, 'т' => 0,
    'У' => 0, 'у' => 0,
    'Ф' => 2, 'ф' => 2,
    'Х' => 0, 'х' => 0,
    'Ц' => 0, 'ц' => 0,
    'Ч' => 0, 'ч' => 0,
    'Ш' => 0, 'ш' => 0,
    'Щ' => 0, 'щ' => 0,
    'Ъ' => 1, 'ъ' => 1,
    'Ы' => 1, 'ы' => 1,
    'Ь' => 1, 'ь' => 1,
    'Э' => 0, 'э' => 0,
    'Ю' => 1, 'ю' => 1,
    'Я' => 1, 'я' => 1,

    '!' => 0,
    '"' => 0,
    '#' => 1,
    '$' => 2,
    '%' => 2,
    '&' => 1,
    '\'' =>  0,
    '(' => 0,
    ')' => 0,
    '*' => 0,
    '+' => 0,
    ',' => 0,
    '-' => 0,
    '.' => 0,
    '/' => 0,
    ':' => 0,
    ';' => 0,
    '<' => 0,
    '=' => 0,
    '>' => 0,
    '?' => 0,
    '@' => 1,
    '[' => 0,
    ']' => 0,
    '^' => 0,
    '_' => 0,
    '`' => 0,
    '{' => 0,
    '|' => 0,
    '}' => 0,
    '~' => 0,
};