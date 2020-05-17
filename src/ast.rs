use crate::token::{ErrCode, Lexer, Num as TokenNum, OpType, Token, TokenTag};

#[allow(dead_code)]
#[derive(Debug)]
enum Value {
    INT(i64),
    BOOL(bool),
    STR(String),
}

trait Expr {
    fn eval(&self) -> Result<Value, AstError>;
}

pub struct And {
    token: Box<dyn Token>,
    args: Vec<Box<dyn Expr>>,
}

impl And {
    fn create(op_tag: Box<dyn Token>, args: Vec<Box<dyn Expr>>) -> Result<And, AstError> {
        Ok(And {
            token: op_tag,
            args: args,
        })
    }
}

impl Expr for And {
    fn eval(&self) -> Result<Value, AstError> {
        let val = true;
        for arg in self.args.iter() {
            let eval_val = arg.eval()?;
            match eval_val {
                Value::INT(i) => {
                    if i == 0 {
                        return Ok(Value::BOOL(false));
                    }
                }
                Value::BOOL(b) => {
                    if !b {
                        return Ok(Value::BOOL(false));
                    }
                }
                _ => {
                    return Err(AstError::FORMAT_NOT_MATCH(
                        "Not correct value format in and operator".to_string(),
                    ));
                }
            }
        }
        return Ok(Value::BOOL(val));
    }
}

pub struct Or {
    token: Box<dyn Token>,
    args: Vec<Box<dyn Expr>>,
}

impl Or {
    fn create(op_tag: Box<dyn Token>, args: Vec<Box<dyn Expr>>) -> Result<Or, AstError> {
        Ok(Or {
            token: op_tag,
            args: args,
        })
    }
}

impl Expr for Or {
    fn eval(&self) -> Result<Value, AstError> {
        let val = false;
        for arg in self.args.iter() {
            let eval_val = arg.eval()?;
            match eval_val {
                Value::INT(i) => {
                    if i == 1 {
                        return Ok(Value::BOOL(true));
                    }
                }
                Value::BOOL(b) => {
                    if !b {
                        return Ok(Value::BOOL(true));
                    }
                }
                _ => {
                    return Err(AstError::FORMAT_NOT_MATCH(
                        "Not correct value format in and operator".to_string(),
                    ));
                }
            }
        }
        return Ok(Value::BOOL(val));
    }
}

pub struct In{
    token: Box<dyn Token>,
    args: Vec<Box<dyn Expr>>,
}

impl In {
    fn create(op_tag: Box<dyn Token>, args: Vec<Box<dyn Expr>>) -> Result<Or, AstError> {
        Ok(Or {
            token: op_tag,
            args: args,
        })
    }
}

impl Expr for In{
    fn eval(&self) -> Result<Value, AstError> {
        //TODO
        if self.args.len() <= 1 {
            return Err(AstError::);
        }
    }
}


pub struct Num {
    token: Box<dyn Token>,
}

impl Num {
    fn create(op_tag: Box<dyn Token>) -> Result<Num, AstError> {
        Ok(Num { token: op_tag })
    }
}

impl Expr for Num {
    fn eval(&self) -> Result<Value, AstError> {
        match self.token.lexeme().parse::<i64>() {
            Ok(i) => {
                return Ok(Value::INT(i));
            }
            Err(_) => {
                return Err(AstError::EVAL_NUM_FAILED("eval number failed!".to_string()));
            }
        }
    }
}

pub struct Str {
    token: Box<dyn Token>,
}

impl Str {
    fn create(op_tag: Box<dyn Token>) -> Result<Str, AstError> {
        Ok(Str { token: op_tag })
    }
}

impl Expr for Str {
    fn eval(&self) -> Result<Value, AstError> {
        return Ok(Value::STR(self.token.lexeme()));
    }
}

pub struct Bool {
    token: Box<dyn Token>,
}

impl Bool {
    fn create(op_tag: Box<dyn Token>) -> Result<Bool, AstError> {
        Ok(Bool { token: op_tag })
    }
}

impl Expr for Bool {
    fn eval(&self) -> Result<Value, AstError> {
        if self.token.lexeme().to_lowercase() == "true" || self.token.lexeme().to_lowercase() == "1"
        {
            return Ok(Value::BOOL(true));
        } else {
            return Ok(Value::BOOL(false));
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    look_token: Option<Box<dyn Token>>,
}

pub enum AstError {
    OTHER(String),
    FORMAT_NOT_MATCH(String),
    LEXER_FAILED(String),
    NOT_MATCH(String),
    NO_TOKEN_MATCH(String),
    NOT_SUPP_OPER(String),
    EVAL_NUM_FAILED(String),
    NOT_ENOUGH_ARGS(String),
}

impl Parser {
    fn create(content: String) -> Result<Parser, AstError> {
        let lexer = Lexer::create(content);
        if lexer.is_err() {
            return Err(AstError::LEXER_FAILED("Lexer init failed!".to_string()));
        }
        Ok(Parser {
            lexer: lexer.unwrap(),
            look_token: None,
        })
    }

    fn parse(&mut self) -> Result<Box<dyn Expr>, AstError> {
        self.match_term(TokenTag::LEFT_BRACKET)?;
        let expr = self.expr()?;
        self.match_term(TokenTag::RIGHT_BRACKET)?;
        return Ok(expr);
    }

    fn expr(&mut self) -> Result<Box<dyn Expr>, AstError> {
        match self.look_token.as_ref() {
            Some(token) => match *token.token_tag() {
                TokenTag::LEFT_BRACKET => {
                    self.move_token()?;
                    match self.look_token.as_ref().unwrap().token_tag() {
                        TokenTag::AND
                        | TokenTag::OR
                        | TokenTag::MOD
                        | TokenTag::EQUALS
                        | TokenTag::IN => {
                            return Ok(self.args_add()?);
                        }
                        _ => {
                            return Err(AstError::NOT_SUPP_OPER(
                                "Not supported operator!".to_string(),
                            ));
                        }
                    }
                }
                TokenTag::NUM => {
                    let token = TokenNum::create_with_token_and_val(TokenTag::NUM, token.lexeme());
                    if token.is_err() {
                        return Err(AstError::OTHER("Create num token failed!".to_string()));
                    }
                    Num::create(token.unwrap())?;
                }
                TokenTag::STR => {}
                TokenTag::VAR => {}
                _ => {}
            },
            None => {
                return Err(AstError::OTHER("Current token is none!".to_string()));
            }
        }
        Err(AstError::OTHER("".to_string()))
    }

    fn args_add(&mut self) -> Result<Box<dyn Expr>, AstError> {
        let mut args: Vec<Box<dyn Expr>> = Vec::new();
        for _ in 0..10000 {
            if !self.move_token()? {
                return Err(AstError::FORMAT_NOT_MATCH(
                    "no right branch packet for and operator but has already went to the end"
                        .to_string(),
                ));
            }
            if self.look_token.is_some()
                && *self.look_token.as_ref().unwrap().token_tag() == TokenTag::RIGHT_BRACKET
            {
                self.move_token()?;
                let and_token = Box::new(OpType {
                    tag: TokenTag::AND,
                    lexeme: "and".to_string(),
                });
                return Ok(Box::new(And::create(and_token, args)?));
            }
            args.push(self.expr()?);
        }
        return Err(AstError::OTHER(
            "Serious problem!!!!!!!!Should not be here".to_string(),
        ));
    }

    fn move_token(&mut self) -> Result<bool, AstError> {
        let scan_result = self.lexer.scan();
        match scan_result {
            Ok(r) => {
                self.look_token.replace(r);
                return Ok(true);
            }
            Err(e) => {
                match e {
                    ErrCode::READ_TO_END(_) => {
                        // Have reached to the end, move will do nothing
                        return Ok(false);
                    }
                    _ => {
                        return Err(AstError::LEXER_FAILED(
                            "Lexer move token failed!".to_string(),
                        ));
                    }
                }
            }
        }
    }

    fn match_term(&mut self, tag: TokenTag) -> Result<(), AstError> {
        match self.look_token.as_ref() {
            Some(s) => {
                if *s.token_tag() == tag {
                    self.move_token()?;
                } else {
                    return Err(AstError::NOT_MATCH(
                        "Expected is not match with current".to_string(),
                    ));
                }
            }
            None => {
                return Err(AstError::NO_TOKEN_MATCH(
                    "There is not token is current parser status".to_string(),
                ));
            }
        }
        Err(AstError::OTHER("".to_string()))
    }
}
