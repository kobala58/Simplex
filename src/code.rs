#![allow(dead_code)]

pub enum ObjectiveType {
    Maximize,
    Minimize,
}
pub enum SolutionType { //definining dvar solution type
    Integer,
    Folating,
    Natural,
}

pub struct TargetFunction {
    vars: Vec<String>,
    coef: Vec<f32>
}

pub struct Constraint{
    name: String, // name of constarint 
    vars: Vec<String>, // dvars 
    coef: Vec<f32>, // vector of coefficients  
}

pub struct Problem {
    name: String,
    pub vars: Vec<String>,
    target: TargetFunction,
    constraints: Vec<Constraint>
}

impl Problem {
    pub fn new() -> Problem {
        Problem {name: "".to_owned(),vars: Vec::new(),target: Target {name: "".to_owned()},constraints: Vec::new()}
    }

    pub fn draw_table(&self){
        todo!()
    } 
    
    pub fn generate_table(&self){
        // let size = 4 + self.coef.len() +
        todo!()
    }

    pub fn generate_index_for_var(&self) -> usize{
        self.vars.len()
    }

    pub fn add_new_variable(&mut self) -> usize{
        let idx = (self.vars.len()+1).to_string();
        let name = format!("x{}",idx);
        self.vars.push(name);

        self.vars.len()
    }

    pub fn add_new_constraint(&mut self) {

    } 
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! constraint {
    ($c1:tt $x1:tt $(+ $c:tt $x:tt)*) => {
        println!("Variable: {}, coef: {}", $x1, $c1);
        $(
        println!("Variable: {}, coef: {}", $x, $c)
    )*
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! problem {
    ($c1:tt $x1:tt $(+ $c:tt)*) => {
    todo!()
    // creating 
    {
            use $crate::code::
        }
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! dvar {
    ($name:tt -> $ttype:path) => {
        {
            use $crate::*;
            let var = $crate::code::Variable::new($name, $ttype);
            
    }};
}

