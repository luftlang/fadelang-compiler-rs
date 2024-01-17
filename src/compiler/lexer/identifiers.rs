use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    let mut buffer = String::new();

    while let Some(peek) = iterator.peek() {
        if !peek.is_alphabetic() {
            break;
        }

        let next = iterator.next()?;

        buffer.push(next);
    }

    if buffer.is_empty() {
        return None;
    }

    Some(LexerToken::Identifier(buffer))
}
