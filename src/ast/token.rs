use std::collections::HashMap;
use std::sync::Arc;
#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenTag {
    AND,
    OR,
    MOD,
    IN,
    EQUALS,
    VAR,
    OTHER,
    NUM,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ErrCode {
    READ_TO_END(String),
    OTHER(String),
}

impl TokenTag {
    #[allow(dead_code)]
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

#[derive(Debug)]
struct OpType {
    tag: TokenTag,
    lexeme: String,
}

impl OpType {
    #[allow(dead_code)]
    fn create_with_token(token_tag: TokenTag, lexeme: String) -> Result<Box<dyn Token>, ErrCode> {
        Ok(Box::new(OpType {
            tag: token_tag,
            lexeme: lexeme,
        }))
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

#[derive(Debug)]
struct Var {
    s: String,
    token_tag: TokenTag,
}

impl Var {
    #[allow(dead_code)]
    fn create_with_token_and_val(
        token_tag: TokenTag,
        s: String,
    ) -> Result<Box<dyn Token>, ErrCode> {
        Ok(Box::new(Var {
            s: s,
            token_tag: token_tag,
        }))
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

#[allow(dead_code)]
#[derive(Debug)]
struct Num {
    token_tag: TokenTag,
    val: i64,
    lexeme: String,
}
impl Num {
    #[allow(dead_code)]
    fn create_with_token_and_val(
        token_tag: TokenTag,
        lexeme: String,
    ) -> Result<Box<dyn Token>, ErrCode> {
        if !lexeme.parse::<i64>().is_ok() {
            println!("lexeme is {}", lexeme);
            return Err(ErrCode::OTHER("Not a number lexeme".to_string()));
        }
        Ok(Box::new(Num {
            token_tag: token_tag,
            val: lexeme.parse::<i64>().unwrap(),
            lexeme: lexeme,
        }))
    }
}

impl Token for Num {
    fn token_tag(&self) -> &TokenTag {
        return &self.token_tag;
    }
    #[allow(dead_code)]
    fn lexeme(&self) -> String {
        return self.lexeme.clone();
    }
}

#[derive(Debug)]
struct Str {
    token_tag: TokenTag,
    s: String,
}

impl Str {
    #[allow(dead_code)]
    fn create_with_token_and_val(
        token_tag: TokenTag,
        s: String,
    ) -> Result<Box<dyn Token>, ErrCode> {
        Ok(Box::new(Str {
            token_tag: token_tag,
            s: s,
        }))
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

#[derive(Debug)]
struct Other {
    token_tag: TokenTag,
    lexeme: String,
}

impl Other {
    #[allow(dead_code)]
    fn create_with_token_and_val(token_tag: TokenTag, s: char) -> Result<Box<dyn Token>, ErrCode> {
        Ok(Box::new(Other {
            token_tag: token_tag,
            lexeme: s.to_string(),
        }))
    }
}
impl Token for Other {
    fn token_tag(&self) -> &TokenTag {
        return &self.token_tag;
    }
    fn lexeme(&self) -> String {
        return self.lexeme.clone();
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Value {
    INT(i64),
    BOOL(bool),
    STR(String),
}

trait Expr {
    fn eval(&self) -> Box<Value>;
}

#[allow(dead_code)]
struct Lexer {
    reserved: HashMap<String, Arc<Box<dyn Token>>>,
    rule_content: String,
    chars: Vec<char>,
    cur_step: i32,
    peek: Option<char>,
}

impl Lexer {
    #[allow(dead_code)]
    fn create(content: String) -> Result<Lexer, ErrCode> {
        let mut reserved: HashMap<String, Arc<Box<dyn Token>>> = HashMap::new();
        let and_ops = OpType::create_with_token(TokenTag::AND, "AND".to_string())?;
        let or_ops = OpType::create_with_token(TokenTag::OR, "OR".to_string())?;
        let mod_ops = OpType::create_with_token(TokenTag::MOD, "MOD".to_string())?;
        let in_ops = OpType::create_with_token(TokenTag::IN, "IN".to_string())?;
        let eq_ops = OpType::create_with_token(TokenTag::EQUALS, "EQUAL".to_string())?;
        reserved.insert(and_ops.lexeme(), Arc::new(and_ops));
        reserved.insert(or_ops.lexeme(), Arc::new(or_ops));
        reserved.insert(mod_ops.lexeme(), Arc::new(mod_ops));
        reserved.insert(in_ops.lexeme(), Arc::new(in_ops));
        reserved.insert(eq_ops.lexeme(), Arc::new(eq_ops));
        let chars: Vec<char> = content.chars().collect();
        Ok(Lexer {
            reserved: reserved,
            rule_content: content,
            cur_step: -1,
            peek: None,
            chars: chars,
        })
    }

    #[allow(dead_code)]
    fn read(step: &mut i32, peek: &mut Option<char>, c: &Vec<char>) -> Result<(), ErrCode> {
        *step += 1;
        match c.get(*step as usize) {
            Some(i) => {
                peek.replace(i.clone());
            }
            None => {
                return Err(ErrCode::READ_TO_END("Has read to the end".to_string()));
            }
        }
        Ok(())
    }
    #[allow(dead_code)]
    fn back_read(
        step: &mut i32,
        peek: &mut Option<char>,
        c: &Vec<char>,
        ori_step: i32,
    ) -> Result<(), ErrCode> {
        loop {
            if *step == ori_step {
                break;
            }
            *step -= 1;
            match c.get(*step as usize) {
                Some(i) => {
                    peek.replace(i.clone());
                }
                None => {
                    return Err(ErrCode::OTHER("Back failed!".to_string()));
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn read_next(&mut self, c: char) -> Result<bool, ErrCode> {
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
    #[allow(dead_code)]
    fn skip_blank_and_read(
        step: &mut i32,
        peek: &mut Option<char>,
        chars: &Vec<char>,
    ) -> Result<(), ErrCode> {
        loop {
            Self::read(step, peek, chars)?;
            let peek = peek.as_ref().unwrap_or(&' ').clone();
            if peek == ' ' || peek == '\t' {
                continue;
            } else {
                break;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn scan(&mut self) -> Result<Box<dyn Token>, ErrCode> {
        Self::skip_blank_and_read(&mut self.cur_step, &mut self.peek, &self.chars)?;
        // 操作符Token匹配
        match self.peek {
            Some('I') => {
                let ori_step = self.cur_step.clone();
                if self.read_next('N')? {
                    return Ok(OpType::create_with_token(TokenTag::IN, "IN".to_string())?);
                } else {
                    Self::back_read(&mut self.cur_step, &mut self.peek, &self.chars, ori_step)?;
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.as_ref().unwrap_or(&' ').clone(),
                    )?);
                }
            }
            Some('M') => {
                let ori_step = self.cur_step.clone();
                if self.read_next('O')? && self.read_next('D')? {
                    return Ok(OpType::create_with_token(TokenTag::MOD, "MOD".to_string())?);
                } else {
                    Self::back_read(&mut self.cur_step, &mut self.peek, &self.chars, ori_step)?;
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.as_ref().unwrap_or(&' ').clone(),
                    )?);
                }
            }
            Some('A') => {
                let ori_step = self.cur_step.clone();
                if self.read_next('N')? && self.read_next('D')? {
                    return Ok(OpType::create_with_token(TokenTag::AND, "AND".to_string())?);
                } else {
                    Self::back_read(&mut self.cur_step, &mut self.peek, &self.chars, ori_step)?;
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.as_ref().unwrap_or(&' ').clone(),
                    )?);
                }
            }
            Some('O') => {
                let ori_step = self.cur_step.clone();
                if self.read_next('R')? {
                    return Ok(OpType::create_with_token(TokenTag::OR, "OR".to_string())?);
                } else {
                    Self::back_read(&mut self.cur_step, &mut self.peek, &self.chars, ori_step)?;
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.as_ref().unwrap_or(&' ').clone(),
                    )?);
                }
            }
            Some('E') => {
                let ori_step = self.cur_step.clone();
                if self.read_next('Q')?
                    && self.read_next('U')?
                    && self.read_next('A')?
                    && self.read_next('L')?
                    && self.read_next('S')?
                {
                    return Ok(OpType::create_with_token(
                        TokenTag::EQUALS,
                        "EQUALS".to_string(),
                    )?);
                } else {
                    Self::back_read(&mut self.cur_step, &mut self.peek, &self.chars, ori_step)?;
                    return Ok(Other::create_with_token_and_val(
                        TokenTag::OTHER,
                        self.peek.as_ref().unwrap_or(&' ').clone(),
                    )?);
                }
            }
            _ => {}
        }
        // Numberic Token analyze
        if self.peek.as_ref().unwrap_or(&' ').clone().is_numeric() {
            let mut v = 0;
            loop {
                v = 10 * v + self.peek.unwrap().to_digit(10 as u32).unwrap();
                let ori_step = self.cur_step.clone();
                Self::read(&mut self.cur_step, &mut self.peek, &self.chars)?;
                if !self.peek.as_ref().unwrap_or(&' ').clone().is_numeric() {
                    Self::back_read(&mut self.cur_step, &mut self.peek, &self.chars, ori_step)?;
                    break;
                }
            }
            return Ok(Num::create_with_token_and_val(
                TokenTag::NUM,
                v.to_string(),
            )?);
        }
        // Var Token analyze
        if self.peek.as_ref().unwrap_or(&' ').clone() == '$' && self.read_next('{')? {
            let mut id = String::new();
            loop {
                Self::read(&mut self.cur_step, &mut self.peek, &self.chars)?;
                let peek_num = self.peek.as_ref().unwrap_or(&' ').clone();
                if peek_num.is_numeric()
                    || (peek_num >= 'a' && peek_num <= 'z')
                    || (peek_num >= 'A' && peek_num <= 'Z')
                {
                    id.push(peek_num);
                } else if peek_num == '}' {
                    return Ok(Var::create_with_token_and_val(TokenTag::VAR, id)?);
                } else {
                    return Err(ErrCode::OTHER(format!(
                            "Illegal arg format for id, id should only contains a-zA-Z0-9, char index:{}",
                            self.cur_step
                        )));
                }
            }
        }
        Ok(Other::create_with_token_and_val(
            TokenTag::OTHER,
            self.peek.unwrap(),
        )?)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_simple_token_split() {
        let mut lexer =
            Lexer::create("( 123 ${i123} IN EQUALS NOT MOD OR AND) ${123} )".to_string()).unwrap();
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        loop {
            let scan_result = lexer.scan();
            if scan_result.is_err() {
                break;
            }
            let scan_result = scan_result.unwrap();
            println!("lexeme: {:?}", scan_result.lexeme());
            tokens.push(scan_result);
        }
        println!("tokens length is:{}", tokens.len());
        assert!(tokens.len() == 14);
    }

    #[test]
    fn test_real_expr_token_rule_content() {
        let mut lexer = Lexer::create("(AND (IN (MOD 12) 1) (IN 1 123 4))".to_string()).unwrap();
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        loop {
            let scan_result = lexer.scan();
            if scan_result.is_err() {
                break;
            }
            let scan_result = scan_result.unwrap();
            println!("lexeme: {:?}", scan_result.lexeme());
            tokens.push(scan_result);
        }
        println!("tokens length is:{}", tokens.len());
        assert!(tokens.len() == 17);
    }

    #[test]
    fn test_not_use_content_expr() {
        let mut lexer = Lexer::create("asdf dsa fda sfE fdf ae 123123 2321 #${123} asdfdsf".to_string()).unwrap();
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        loop {
            let scan_result = lexer.scan();
            if scan_result.is_err() {
                break;
            }
            let scan_result = scan_result.unwrap();
            println!("lexeme: {:?}", scan_result.lexeme());
            tokens.push(scan_result);
        }
        println!("tokens length is:{}", tokens.len());
        assert!(tokens.len() == 29);
    }
}
