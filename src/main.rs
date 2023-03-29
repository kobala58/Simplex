use crate::code::{Problem, SolutionType, ObjectiveType};

pub mod code;


fn main () {
    constraint!(3 "x1" + 4 "x2");
    let mut test = Problem::new();
    let x1 = test.add_new_variable(); // return index of this dvar
    let x2 = test.add_new_variable();
    test.set_target(problem!(3 x1 + 4 x2 -> ObjectiveType::Maximize)); 
}
