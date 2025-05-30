use std::string::String;

const RESERVED_WORDS: &[&'static str] = &[
    "ABSOLUTE",
    "AND",
    "ARRAY",
    "ASM",
    "BEGIN",
    "CASE",
    "CONST",
    "CONSTRUCTOR",
    "DESTRUCTOR",
    "DIV",
    "DO",
    "DOWNTO",
    "ELSE",
    "END",
    "FILE",
    "FOR",
    "FUNCTION",
    "GOTO",
    "IF",
    "IMPLEMENTATION",
    "IN",
    "INHERITED",
    "INLINE",
    "INTERFACE",
    "LABEL",
    "MOD",
    "NIL",
    "NOT",
    "OBJECT",
    "OF",
    "OPERATOR",
    "OR",
    "PACKED",
    "PROCEDURE",
    "PROGRAM",
    "RECORD",
    "REINTRODUCE",
    "REPEAT",
    "SELF",
    "SET",
    "SHL",
    "SHR",
    "STRING",
    "THEN",
    "TO",
    "TYPE",
    "UNIT",
    "UNTIL",
    "USES",
    "VAR",
    "WHILE",
    "WITH",
    "XOR",
];

pub fn is_reserved(s: &str) -> bool {
    let uppercase = String::from(s).to_uppercase();

    for reserved_word in RESERVED_WORDS {
        if String::from(*reserved_word) == uppercase {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_case_insensitive() {
        let result = is_reserved("enD");
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_match_reserved_words() {
        let result = is_reserved("while");
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_ignore_non_reserved_words() {
        let result = is_reserved("potato");
        assert_eq!(result, false);
    }
}
