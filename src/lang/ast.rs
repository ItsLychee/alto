use std::collections::HashMap;

use super::lexer::Token;



// Scope is the current state of execution
struct Scope {
    variable_table: HashMap<String, String>

}


pub trait Node {
    fn eval(&self) -> Option<String>;
    fn lexemes(&self) -> Vec<Token>;
    fn nodes(&self) -> Vec<Box<dyn Node>>;
}





