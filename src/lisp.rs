use std::collections::HashMap;
use std::sync::Arc;

enum TokenTag {
    AND,
    OR,
    MOD,
    IN,
    EQUALS,
    VAR,
    OTHER,
    NUM,
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
            TokenTag::OTHER => 262,
            TokenTag::NUM => 263,
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

struct Lexer {
    reserved: HashMap<String, Arc<Box<dyn Token>>>,
    rule_content: String,
    chars: Vec<char>,
    cur_step: i32,
    peek: Option<char>,
}

impl Lexer {
    fn create(content: String) -> Lexer {
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
        let chars: Vec<char> = content.chars().collect();
        Lexer {
            reserved: reserved,
            rule_content: content,
            cur_step: -1,
            peek: None,
            chars: chars,
        }
    }

    fn read(step: &mut i32, peek: &mut Option<char>, c: &Vec<char>) -> Result<(), String> {
        *step += 1;
        match c.get(*step as usize) {
            Some(i) => {
                peek.replace(i.clone());
            }
            None => {
                return Err("Has read to the end".to_string());
            }
        }
        Ok(())
    }

    fn read_next(&mut self, c: char) -> Result<bool, String> {
        Self::read(&mut self.cur_step, &mut self.peek, &self.chars)?;
        // when return true, will go to next char
        // so set this char to ' '
        if self.peek == Some(c) {
            self.peek = Some(' ');
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /**
     * Skip all the blank chars
     */
    fn skip_blank_and_read(
        step: &mut i32,
        peek: &mut Option<char>,
        chars: &Vec<char>,
    ) -> Result<(), String> {
        loop {
            Self::read(step, peek, chars)?;
            let peek = peek.unwrap_or(' ');
            if peek == ' ' || peek == '\t' {
                continue;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn scan(&mut self) -> Result<Box<dyn Token>, String> {
        Self::skip_blank_and_read(&mut self.cur_step, &mut self.peek, &self.chars)?;
        // 操作符Token匹配
        match self.peek {
            Some('I') => {
                if self.read_next('N')? {
                    return Ok(OpType::create_with_token(TokenTag::IN, "IN".to_string()));
                } else {
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.unwrap_or(' '),
                    ));
                }
            }
            Some('M') => {
                if self.read_next('O')? && self.read_next('D')? {
                    return Ok(OpType::create_with_token(TokenTag::MOD, "MOD".to_string()));
                } else {
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.unwrap_or(' '),
                    ));
                }
            }
            Some('A') => {
                if self.read_next('N')? && self.read_next('D')? {
                    return Ok(OpType::create_with_token(TokenTag::MOD, "AND".to_string()));
                } else {
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.unwrap_or(' '),
                    ));
                }
            }
            Some('O') => {
                if self.read_next('R')? {
                    return Ok(OpType::create_with_token(TokenTag::IN, "OR".to_string()));
                } else {
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.unwrap_or(' '),
                    ));
                }
            }
            Some('E') => {
                if self.read_next('Q')?
                    && self.read_next('U')?
                    && self.read_next('A')?
                    && self.read_next('L')?
                {
                    return Ok(OpType::create_with_token(TokenTag::MOD, "AND".to_string()));
                } else {
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.unwrap_or(' '),
                    ));
                }
            }
            _ => {}
        }
        // Numberic Token analyze
        if self.peek.unwrap_or(' ').is_numeric() {
            let mut v = 0;
            loop {
                v = 10 * v + self.peek.unwrap().to_digit(10 as u32).unwrap();
                Self::read(&mut self.cur_step, &mut self.peek, &self.chars)?;
                if !self.peek.unwrap_or(' ').is_numeric() {
                    break;
                }
            }
            return Ok(Num::create_with_token_and_val(
                TokenTag::NUM,
                self.peek.unwrap().to_string(),
            ));
        }
        // Var Token analyze
        if self.peek.unwrap_or(' ') == '$' && self.read_next('{')? {
            let mut id = String::new();
            loop {
                Self::read(&mut self.cur_step, &mut self.peek, &self.chars)?;
                let peek_num = self.peek.unwrap_or(' ');
                if self.peek.unwrap_or(' ').is_numeric()
                    || (peek_num >= 'a' && peek_num <= 'z')
                    || (peek_num >= 'A' && peek_num <= 'Z')
                {
                    id.push(peek_num);
                    break;
                } else if peek_num == '}' {
                    return Ok(Var::create_with_token_and_val(TokenTag::VAR, id));
                } else {
                    return Err(format!(
                            "Illegal arg format for id, id should only contains a-zA-Z0-9, char index:{}",
                            self.cur_step
                        ));
                }
            }
            return Ok(Num::create_with_token_and_val(
                TokenTag::NUM,
                self.peek.unwrap().to_string(),
            ));
        }
        Ok(Other::create_with_token_and_val(TokenTag::OTHER, self.peek.unwrap()));
    }
}
