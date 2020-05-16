use std::collections::HashMap;
use std::sync::Arc;

enum TokenTag {
    AND,
    OR,
    MOD,
    IN,
    EQUALS,
    VAR,
}

impl TokenTag {
    fn value(&self) -> i32 {
        match *self {
            TokenTag::AND => 256,
            TokenTag::OR => 257,
            TokenTag::MOD => 258,
            TokenTag::IN => 259,
            TokenTag::EQUALS => 260,
            TokenTag::VAR => 261,
        }
    }
}

trait Token {
    fn token_tag(&self) -> &TokenTag;
    fn lexeme(&self) -> String;
}

struct OpType {
    tag: TokenTag,
    lexeme: String,
}

impl OpType {
    fn create_with_token(token_tag: TokenTag, lexeme: String) -> Box<dyn Token> {
        Box::new(OpType {
            tag: token_tag,
            lexeme: lexeme,
        })
    }
}

impl Token for OpType {
    fn token_tag(&self) -> &TokenTag {
        return &self.tag;
    }
    fn lexeme(&self) -> String {
        return self.lexeme.clone();
    }
}

struct Var {
    s: String,
    token_tag: TokenTag,
}

impl Var {
    fn create_with_token_and_val(token_tag: TokenTag, s: String) -> Box<dyn Token> {
        Box::new(Var {
            s: s,
            token_tag: token_tag,
        })
    }
}

impl Token for Var {
    fn token_tag(&self) -> &TokenTag {
        return &self.token_tag;
    }
    fn lexeme(&self) -> String {
        return self.s.clone();
    }
}

struct Num {
    token_tag: TokenTag,
    val: i64,
    lexeme: String,
}
impl Num {
    fn create_with_token_and_val(token_tag: TokenTag, lexeme: String) -> Box<dyn Token> {
        Box::new(Num {
            token_tag: token_tag,
            val: lexeme.parse::<i64>().map_or_else(|e| -1 as i64, |v| v),
            lexeme: lexeme,
        })
    }
}

impl Token for Num {
    fn token_tag(&self) -> &TokenTag {
        return &self.token_tag;
    }
    fn lexeme(&self) -> String {
        return self.lexeme().clone();
    }
}

struct Str {
    token_tag: TokenTag,
    s: String,
}

impl Str {
    fn create_with_token_and_val(token_tag: TokenTag, s: String) -> Box<dyn Token> {
        Box::new(Str {
            token_tag: token_tag,
            s: s,
        })
    }
}

impl Token for Str {
    fn token_tag(&self) -> &TokenTag {
        return &self.token_tag;
    }
    fn lexeme(&self) -> String {
        return self.s.clone();
    }
}

struct Other {
    token_tag: TokenTag,
    lexeme: String,
}

impl Other {
    fn create_with_token_and_val(token_tag: TokenTag, s: char) -> Box<dyn Token> {
        Box::new(Str {
            token_tag: token_tag,
            s: s.to_string(),
        })
    }
}

enum Value {
    INT(i64),
    BOOL(bool),
    STR(String),
}

trait Expr {
    fn eval(&self) -> Box<Value>;
}

struct Lexer<'a> {
    reserved: HashMap<String, Arc<Box<dyn Token>>>,
    rule_content: String,
    chars: Vec<char>,
    cur_step: i32,
    // current readed char
    peek: Option<&'a char>,
}

impl<'a> Lexer<'a> {
    fn create(content: String) -> Lexer<'a> {
        let mut reserved: HashMap<String, Arc<Box<dyn Token>>> = HashMap::new();
        let and_ops = OpType::create_with_token(TokenTag::AND, "AND".to_string());
        let or_ops = OpType::create_with_token(TokenTag::OR, "OR".to_string());
        let mod_ops = OpType::create_with_token(TokenTag::MOD, "MOD".to_string());
        let in_ops = OpType::create_with_token(TokenTag::IN, "IN".to_string());
        let eq_ops = OpType::create_with_token(TokenTag::EQUALS, "EQUAL".to_string());
        reserved.insert(and_ops.lexeme(), Arc::new(and_ops));
        reserved.insert(or_ops.lexeme(), Arc::new(or_ops));
        reserved.insert(mod_ops.lexeme(), Arc::new(mod_ops));
        reserved.insert(in_ops.lexeme(), Arc::new(in_ops));
        reserved.insert(eq_ops.lexeme(), Arc::new(eq_ops));
        Lexer {
            reserved: reserved,
            chars: content.chars().collect(),
            rule_content: content,
            cur_step: -1,
            peek: None,
        }
    }

    fn read_next(&'a mut self) {
        self.cur_step += 1;
        self.peek = self.chars.get(self.cur_step as usize);
    }

    fn read_next(&mut self, c: char) -> bool {
        false
    }
}
