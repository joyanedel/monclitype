use std::iter::zip;

use itertools::Itertools;

use crate::{
    sentences::build_sentence,
    types::{GameFinished, KeyEventSource, WordGameStatus, WordMatch},
};

pub fn get_current_game_status(
    events: &Vec<KeyEventSource>,
    target_sentence: &str,
) -> Result<WordGameStatus, GameFinished> {
    let input_sentence = build_sentence(&events);
    let input_words = input_sentence.split(" ").collect_vec();
    let input_words_length = input_words.len();

    let target_words = target_sentence.split(" ").collect_vec();
    let target_words_length = target_words.len();

    if input_words_length > target_words_length {
        return Err(GameFinished);
    }

    let mut already_written_words = zip(
        input_words[..input_words_length].into_iter(),
        target_words[..input_words_length].into_iter(),
    )
    .map(|(&a, &b)| zip_input_target_word(a, b))
    .collect_vec();
    let current_written_word = already_written_words.pop().unwrap();
    let not_written_sentence = target_words[input_words_length..].into_iter().join(" ");

    Ok(WordGameStatus(
        already_written_words,
        current_written_word,
        if not_written_sentence.len() == 0 {
            None
        } else {
            Some(not_written_sentence)
        },
    ))
}

pub fn zip_input_target_word(input_sentence: &str, target_sentence: &str) -> WordMatch {
    input_sentence
        .chars()
        .zip_longest(target_sentence.chars())
        .collect()
}

#[cfg(test)]
mod current_game_status_tests {
    use std::time::Instant;

    use crossterm::event::KeyCode;
    use itertools::EitherOrBoth;

    use crate::types::{KeyEventSource, WordGameStatus};

    use super::get_current_game_status;

    #[test]
    fn game_with_0_words_completed() {
        let expected = WordGameStatus(
            vec![],
            vec![
                EitherOrBoth::Right('h'),
                EitherOrBoth::Right('e'),
                EitherOrBoth::Right('l'),
                EitherOrBoth::Right('l'),
                EitherOrBoth::Right('o'),
            ]
            .into(),
            "world".to_string().into(),
        );

        let result = get_current_game_status(&vec![], "hello world");

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn game_in_last_word() {
        let word_1 = vec![
            EitherOrBoth::Both('h', 'h'),
            EitherOrBoth::Both('e', 'e'),
            EitherOrBoth::Both('l', 'l'),
            EitherOrBoth::Both('l', 'l'),
            EitherOrBoth::Both('o', 'o'),
        ];
        let word_2 = vec![
            EitherOrBoth::Both('w', 'w'),
            EitherOrBoth::Both('o', 'o'),
            EitherOrBoth::Both('r', 'r'),
            EitherOrBoth::Both('l', 'l'),
            EitherOrBoth::Both('d', 'd'),
        ];
        let timestamp = Instant::now();
        let events = vec![
            KeyEventSource {
                key: KeyCode::Char('h'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('e'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('l'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('l'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('o'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char(' '),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('w'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('o'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('r'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('l'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('d'),
                timestamp,
            },
        ];
        let expected = WordGameStatus(vec![word_1], word_2, None);

        let result = get_current_game_status(&events, "hello world");

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn game_finished() {
        let result = get_current_game_status(
            &vec![KeyEventSource {
                key: KeyCode::Char(' '),
                timestamp: Instant::now(),
            }],
            "",
        );

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod zip_input_target_word_tests {
    use itertools::EitherOrBoth;

    use super::zip_input_target_word;

    #[test]
    fn right_match_words() {
        let word = "hw";

        let result = zip_input_target_word(word, word);

        assert_eq!(
            result,
            vec![EitherOrBoth::Both('h', 'h'), EitherOrBoth::Both('w', 'w'),]
        )
    }

    #[test]
    fn input_is_larger() {
        let input_word = "hw";
        let target_word = "h";

        let result = zip_input_target_word(input_word, target_word);

        assert_eq!(
            result,
            vec![EitherOrBoth::Both('h', 'h'), EitherOrBoth::Left('w')]
        );
    }

    #[test]
    fn target_is_larger() {
        let input_word = "h";
        let target_word = "hw";

        let result = zip_input_target_word(input_word, target_word);

        assert_eq!(
            result,
            vec![EitherOrBoth::Both('h', 'h'), EitherOrBoth::Right('w'),]
        );
    }
}
