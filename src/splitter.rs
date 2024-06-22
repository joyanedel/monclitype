use itertools::Itertools;

use crate::types::{WordGameStatus, WordMatch};

pub fn get_current_game_status() -> WordGameStatus {
    WordGameStatus(vec![], vec![], String::new())
}

pub fn zip_input_target_word(input_sentence: &str, target_sentence: &str) -> WordMatch {
    input_sentence
        .chars()
        .zip_longest(target_sentence.chars())
        .collect()
}

#[cfg(test)]
mod current_game_status_tests {}

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
