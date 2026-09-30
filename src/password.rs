use std::char;

const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'z',
];
const CHUNK_COUNT: usize = 3;
const CHUNK_LEN: usize = 6;

pub fn generate_password() -> String {
    let create_chunks = || {
        (0..CHUNK_LEN)
            .map(|i| {
                if i == 1 || i == 4 {
                    VOWELS[rand::random_range(0..VOWELS.len())]
                } else {
                    CONSONANTS[rand::random_range(0..CONSONANTS.len())]
                }
            })
            .collect::<String>()
    };

    add_number(&add_uppercase_letter(&format!(
        "{}-{}-{}",
        create_chunks(),
        create_chunks(),
        create_chunks()
    )))
}

fn add_number(password: &str) -> String {
    let mut chunks = password.split('-').collect::<Vec<&str>>();

    let number_chunk = rand::random_range(0..CHUNK_COUNT);

    let mut chosen_chunk = chunks[number_chunk].chars().collect::<Vec<char>>();

    let valid_indexes = chosen_chunk
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.is_ascii_uppercase())
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let number_index = valid_indexes[rand::random_range(0..valid_indexes.len())];

    chosen_chunk[number_index] = char::from_digit(rand::random_range(0..10), 10).unwrap();

    let modified_chunk = chosen_chunk.iter().collect::<String>();

    chunks[number_chunk] = &modified_chunk;

    chunks.join("-")
}

fn add_uppercase_letter(password: &str) -> String {
    let mut chunks = password.split('-').collect::<Vec<&str>>();

    let upper_chunk = rand::random_range(0..CHUNK_COUNT);

    let mut chosen_chunk = chunks[upper_chunk].chars().collect::<Vec<char>>();

    let valid_indexes = chosen_chunk
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let upper_index = valid_indexes[rand::random_range(0..valid_indexes.len())];

    chosen_chunk[upper_index] = chosen_chunk[upper_index].to_ascii_uppercase();

    let modified_chunk = chosen_chunk.iter().collect::<String>();

    chunks[upper_chunk] = &modified_chunk;

    chunks.join("-")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password() {
        let password = generate_password();

        // Has an uppercase letter
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));

        // Has a digit
        assert!(password.chars().any(|c| c.is_ascii_digit()));

        // Has 20 characters
        assert!(password.len() == 20);
    }
}
