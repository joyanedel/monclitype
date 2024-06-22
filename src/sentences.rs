use crossterm::event::KeyCode;

use crate::types::KeyEventSource;

pub fn build_sentence(key_events: &Vec<KeyEventSource>) -> String {
    key_events.iter().fold(String::new(), sentence_reducer)
}

fn sentence_reducer(acc: String, new_event: &KeyEventSource) -> String {
    match new_event.key {
        KeyCode::Char(' ') => {
            if let Some(' ') = acc.chars().last() {
                acc
            } else {
                format!("{} ", acc)
            }
        }
        KeyCode::Char(v) => format!("{}{}", acc, v),
        KeyCode::Backspace => {
            if acc.len() == 0 {
                acc
            } else {
                acc[..acc.len() - 1].to_owned()
            }
        }
        _ => acc,
    }
}

pub fn verify_sentence_input(input_sentence: &str, target_sentence: &str) -> bool {
    input_sentence.eq(target_sentence)
}

#[cfg(test)]
mod build_sentence_tests {
    use crate::types::KeyEventSource;
    use crossterm::event::KeyCode;
    use std::time::Instant;

    use super::build_sentence;

    #[test]
    fn empty_sentence() {
        let events = vec![];

        let result = build_sentence(&events);

        assert_eq!(result.as_str(), "");
    }

    #[test]
    fn non_empty_sentence() {
        let timestamp = Instant::now();
        let events = vec![
            KeyEventSource {
                key: KeyCode::Char('h'),
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
        ];

        let result = build_sentence(&events);

        assert_eq!(result.as_str(), "h w");
    }

    #[test]
    fn non_empty_sentence_with_backspace() {
        let timestamp = Instant::now();
        let events = vec![
            KeyEventSource {
                key: KeyCode::Char('h'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('w'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Backspace,
                timestamp,
            },
        ];

        let result = build_sentence(&events);

        assert_eq!(result.as_str(), "h");
    }

    #[test]
    fn empty_sentence_with_overuse_of_backspaces() {
        let timestamp = Instant::now();
        let events = vec![
            KeyEventSource {
                key: KeyCode::Char('h'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char('w'),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Backspace,
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Backspace,
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Backspace,
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Backspace,
                timestamp,
            },
        ];

        let result = build_sentence(&events);

        assert_eq!(result.as_str(), "");
    }

    #[test]
    fn overuse_of_spaces() {
        let timestamp = Instant::now();
        let events = vec![
            KeyEventSource {
                key: KeyCode::Char(' '),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char(' '),
                timestamp,
            },
            KeyEventSource {
                key: KeyCode::Char(' '),
                timestamp,
            },
        ];

        let result = build_sentence(&events);

        assert_eq!(result.as_str(), " ");
    }
}
