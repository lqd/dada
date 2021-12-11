use crate::word::Word;
use crate::{token_tree, Db};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    /// "foo", could be keyword or an identifier
    Alphabetic(Word),

    /// 22_000
    Number(Word),

    /// An operator like `+` that is NOT followed by another operator.
    OpAlone(char),

    /// An operator like `+` that IS followed by another operator.
    OpAdjacent(char),

    /// `(`, `)`, `[`, `]`, `{`, or `}`
    Delimiter(char),

    /// When we encounter an opening delimiter, all the contents up to (but not including)
    /// the closing delimiter are read into a Tree.
    Tree(token_tree::TokenTree),

    /// Some whitespace (` `, `\n`, etc)
    Whitespace(char),

    /// Some unclassifiable, non-whitespace char
    Unknown(char),
}

impl Token {
    pub fn span_len(self, db: &dyn Db) -> u32 {
        match self {
            Token::Tree(tree) => tree.span(db).len(),
            Token::Alphabetic(word) | Token::Number(word) => {
                word.as_str(db).len().try_into().unwrap()
            }
            Token::Delimiter(ch)
            | Token::OpAlone(ch)
            | Token::OpAdjacent(ch)
            | Token::Whitespace(ch)
            | Token::Unknown(ch) => ch.len_utf8().try_into().unwrap(),
        }
    }

    pub fn alphabetic(self) -> Option<Word> {
        match self {
            Token::Alphabetic(word) => Some(word),
            _ => None,
        }
    }

    pub fn alphabetic_str(self, db: &dyn Db) -> Option<&str> {
        self.alphabetic().map(|i| i.as_str(db))
    }

    /// Returns Some if this is a [`TokenTree::Tree`] variant.
    pub fn tree(self) -> Option<token_tree::TokenTree> {
        match self {
            Token::Tree(tree) => Some(tree),
            _ => None,
        }
    }
}
