use crate::dictionary;

pub fn en_to_mn(value: &str) -> String {
    let mut new_value: String = String::from("");
    let mut ignore: bool = false;
    let mut found_multi: bool = false;

    value
        .chars()
        .into_iter()
        .enumerate()
        .for_each(|(index, character)| {
            if found_multi == true {
                found_multi = false;
                return;
            }

            if character == '`' {
                ignore = !ignore;
                return;
            }

            if ignore == true {
                new_value.push(character);
                return;
            }

            match value.chars().nth(index + 1) {
                Some(next_character) => {
                    let check_str = format!("{}{}", character, next_character);

                    match dictionary::TRANSFER_DICTIONARY.get(&check_str) {
                        Some(char) => {
                            new_value.push_str(char);
                            found_multi = true;
                        }
                        None => match dictionary::TRANSFER_DICTIONARY.get(&character.to_string()) {
                            Some(single_char) => {
                                new_value.push_str(single_char);
                            }
                            None => {
                                new_value.push(character);
                            }
                        },
                    }
                }
                None => match dictionary::TRANSFER_DICTIONARY.get(&character.to_string()) {
                    Some(char) => {
                        new_value.push_str(char);
                    }
                    None => {
                        new_value.push(character);
                    }
                },
            }
        });

    new_value
}
