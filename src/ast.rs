use crate::token::{
    ErrCode, Lexer, Num as TokenNum, OpType, Str as TokenStr, Token, TokenTag, Var as TokenVar,
};
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum Value {
    INT(i64),
    BOOL(bool),
    STR(String),
}

trait Expr {
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError>;
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
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
        let val = true;
        for arg in self.args.iter() {
            let eval_val = arg.eval(ctx.clone())?;
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

pub struct Mod {
    token: Box<dyn Token>,
    args: Vec<Box<dyn Expr>>,
}

impl Mod {
    fn create(op_tag: Box<dyn Token>, args: Vec<Box<dyn Expr>>) -> Result<Mod, AstError> {
        Ok(Mod {
            token: op_tag,
            args: args,
        })
    }
}

impl Expr for Mod {
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
        if self.args.len() < 2 {
            return Err(AstError::NOT_ENOUGH_ARGS(
                "Mod does not have enough args!".to_string(),
            ));
        }
        let arg0 = self.args.get(0);
        let arg1 = self.args.get(1);
        if arg0.is_none() || arg1.is_none() {
            return Err(AstError::NOT_ENOUGH_ARGS(
                "Args in mod has noe value".to_string(),
            ));
        }
        let arg0 = arg0.unwrap().eval(ctx.clone())?;
        let arg1 = arg1.unwrap().eval(ctx.clone())?;

        if let Value::INT(i1) = arg0 {
            if let Value::INT(i2) = arg1 {
                let result = i1 % i2;
                return Ok(Value::INT(result));
            }
        }
        return Err(AstError::ARG_NOT_CORRECT(
            "Arg's format is not correct for mod ".to_string(),
        ));
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
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
        let val = false;
        for arg in self.args.iter() {
            let eval_val = arg.eval(ctx.clone())?;
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

pub struct In {
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

impl Expr for In {
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
        if self.args.len() <= 1 {
            return Err(AstError::NOT_ENOUGH_ARGS(
                "In operator should have at least two arguments".to_string(),
            ));
        }
        let arg0 = self.args.get(0);
        if arg0.is_none() {
            return Ok(Value::BOOL(false));
        }
        let arg0 = arg0.unwrap().eval(ctx.clone())?;
        for i in 1..(self.args.len() - 1) {
            let arg = self.args.get(i);
            if arg.is_some() {
                let arg = arg.unwrap().eval(ctx.clone())?;
                if arg0 == arg {
                    return Ok(Value::BOOL(true));
                }
            }
        }
        return Ok(Value::BOOL(false));
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
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
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
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
        return Ok(Value::STR(self.token.lexeme()));
    }
}
pub struct Var {
    token: Box<dyn Token>,
}

impl Var {
    fn create(op_tag: Box<dyn Token>) -> Result<Var, AstError> {
        Ok(Var { token: op_tag })
    }
}

impl Expr for Var {
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
        let key = self.token.lexeme();
        let val = ctx.get(&key);
        if val.is_none() {
            return Ok(Value::BOOL(false));
        } else {
            return Ok(val.unwrap().clone());
        }
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
    fn eval(&self, ctx: Arc<HashMap<String, Value>>) -> Result<Value, AstError> {
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
    ARG_NOT_CORRECT(String),
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
                TokenTag::STR => {
                    let token = TokenStr::create_with_token_and_val(TokenTag::STR, token.lexeme());
                    if token.is_err() {
                        return Err(AstError::OTHER("Create str token failed!".to_string()));
                    }
                    Str::create(token.unwrap())?;
                }
                TokenTag::VAR => {
                    let token = TokenVar::create_with_token_and_val(TokenTag::VAR, token.lexeme());
                    if token.is_err() {
                        return Err(AstError::OTHER("Create str token failed!".to_string()));
                    }
                    Var::create(token.unwrap())?;
                }
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
