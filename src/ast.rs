#![allow(dead_code,unused_variables)]

use std::collections::HashMap;

/// data types
pub enum Type {
	Class(Class), //for a class that has no state
	Instance(Class), //for a class with state 
	Function(Function), //a function
	String(String), //a generic string type
	Num(isize), //a generic number type
	Variable(String), //a refrence to another type
	Step(Box<Step>)
}

/// math operations
pub enum MathOperation {
	Add,  // A+B
	Sub,  // A-B
	Mul,  // A*B
	Div,  // A/B
	Mod,  // A%B
	Pow,  // A^B
}


/// operations
pub enum Operation {
	LessThan,    // A<B
	Equal,       // A=B (but the entire thing)
	GreaterThan, // A>B
	And,         // A&&B
	Or,          // A||B
	SameType     // A=B (but the Type)
}

/// the possible steps
pub enum Step{
	Import(String,Option<String>), // IMPORT <String> <Option<String>> 
	Calln(String,Vec<Type>),       // CALLN <String> <Vec<Type>>,
	Call(String,Class),             // CALL <String> <Class>
	Return(Type),                  // RETURN <Type>
	If(Type,usize),                // jumps to the step usize if isize is <= 0
	Op(Type,Operation,Type),	   // OP <Type> <Operation> <Type>
	Math(Type,MathOperation,Type), // MATH <Type> <Operation> <Type>
	Set(Type,Type)				   // SET <Variable> <Type>
}

/// a function 
pub struct Function {
	inputs: Vec<Type>, // a list of the types
	steps: Vec<Step>,  // a list of the steps in the function
	name: String       // the name of the function
}

/// a class that has fields and a name
pub struct Class {
	/// the fields of the Class
	fields: HashMap<String,Type>,
	/// the name of the Class
	name: String                  
}

fn test() {
	// this would print
	// hello!
	// -128
	let steps = vec![
		Step::Calln("print".to_string(),vec![
			Type::String("hello!".to_string())
		]),
		Step::Set(
			Type::Variable("neat".to_string()),
			Type::Num(128)
		),
		Step::Math(Type::Variable("neat".to_string()),MathOperation::Add,Type::Num(-256)),
		Step::Calln("print".to_string(),vec![
			Type::Variable("neat".to_string())
		])
	];

}