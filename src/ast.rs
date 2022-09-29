#![allow(dead_code)]

use std::collections::HashMap;

enum Type {
	Class(Class),
	Function(Function),
	String(String),
	Num(isize)
}

enum Operation {
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Pow,
}

enum Step{
	Import(String,Option<String>), //IMPORT <String> <Option<String>> 
	Calln(String,Vec<Type>), //CALLN <String> <Vec<Type>>,
	Call(String,Class), //CALL <String> <Class>
	Return(Type), //RETURN <Type>
	If(isize,usize) // jumps to the step usize if isize is <= 0
}

struct Function {
	inputs: Vec<Type>,
	steps: Vec<Step>, //TODO: replace with a proper type indicating actuall function steps
	name: String
}

struct Class {
	fields: HashMap<String,Type>,
	name: String
}