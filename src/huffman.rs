#![allow(dead_code)]

use bitvec::{prelude::*, macros::internal::funty::Fundamental};

use std::{fmt, collections::BTreeMap};

/// Keywords in my languague
#[derive(Debug,Eq,PartialEq,Clone,Hash,Ord,PartialOrd)]
pub enum Keywords{
	INVALID, // DO NOT USE, literally exist to say "INVALID internal keyword"
	CALL,
	CALLN,
	STR,
	VAR,
	IF,
	OP,
	DEF,
	CDEF,
	EOF,
	ES,
	LBL,
	GOTO,
	MATH,
	IMPORT
}

impl Keywords {
	fn from_usize(size: usize) -> Keywords {
		match size {
			0 => { Keywords::INVALID}
			1 => { Keywords::CALL}
			2 => { Keywords::CALLN}
			3 => { Keywords::STR}
			4 => { Keywords::VAR}
			5 => { Keywords::IF}
			6 => { Keywords::OP}
			7 => { Keywords::DEF}
			8 => { Keywords::CDEF}
			9 => { Keywords::EOF}
			10 => { Keywords::ES}
			11 => { Keywords::LBL}
			12 => { Keywords::GOTO}
			13 => { Keywords::MATH}
			14 => { Keywords::IMPORT}			
			_ => { Keywords::INVALID }
		}
	}
	fn from_string(string: &str) -> Keywords {
		match string {
			"INVALID" => { Keywords::INVALID}
			"CALL" => { Keywords::CALL}
			"CALLN" => { Keywords::CALLN}
			"STR" => { Keywords::STR}
			"VAR" => { Keywords::VAR}
			"IF" => { Keywords::IF}
			"OP" => { Keywords::OP}
			"DEF" => { Keywords::DEF}
			"CDEF" => { Keywords::CDEF}
			"EOF" => { Keywords::EOF}
			"ES" => { Keywords::ES}
			"LBL" => { Keywords::LBL}
			"GOTO" => { Keywords::GOTO}
			"MATH" => { Keywords::MATH}
			"IMPORT" => { Keywords::IMPORT}			
			_ => { Keywords::INVALID }
		}
	}
	fn to_string(&self) -> String {
		match self {
			Keywords::INVALID => { "INVALID" }
			Keywords::CALL => { "CALL" }
			Keywords::CALLN => { "CALLN" }
			Keywords::STR => { "STR" }
			Keywords::VAR => { "VAR" }
			Keywords::IF => { "IF" }
			Keywords::OP => { "OP" }
			Keywords::DEF => { "DEF" }
			Keywords::CDEF => { "CDEF" }
			Keywords::EOF => { "EOF" }
			Keywords::ES => { "ES" }
			Keywords::LBL => { "LBL" }
			Keywords::GOTO => { "GOTO" }
			Keywords::MATH => { "MATH" }
			Keywords::IMPORT => { "IMPORT" }			
			_ => { "INVALID" }
		}.to_string()
	}
}

/// Possible Values of Nodes
#[derive(Debug,Eq,PartialEq,Clone,Hash,Ord,PartialOrd)]
pub enum Value {
	Str(String), //for a value which is not specifically in our keywords
	Keyword(Keywords), //for a value which is a number representation of our keywords
	None //for if you try to get a value our of a node which does not have any value
}

/// A node in a binary tree
#[derive(Eq,PartialEq)]
pub struct Node {
	value: Value,
	left_node: Option<Box<Node>>,
	right_node: Option<Box<Node>>,
	weight: usize
}

pub trait BitHelp {
	fn write_bit(&mut self, bit: bool);
	fn write_number(&mut self,num: usize);
	fn write_string(&mut self,string: String);
	
	fn read_bit(&mut self,) -> bool;
	fn read_number(&mut self) -> usize;
	fn read_string(&mut self) -> String;
}

impl BitHelp for BitVec<u8> {
	//write
	fn write_bit(&mut self,bit: bool) {
		self.push(bit)
	}

	fn write_number(&mut self,num: usize) {
		//determine whether we are fitting in 8,16,32,64 bits
		if num <= 255 {
			//write a 8-bit number
			self.write_bit(false);
			self.write_bit(false);
			let mut num_bit_vec = num.as_u8().view_bits::<Lsb0>().to_bitvec();
			self.append(&mut num_bit_vec);
		} else if num <=65535 {
			//write a 16 bit number
			self.write_bit(true);
			self.write_bit(false);
			let mut num_bit_vec = num.as_u16().view_bits::<Lsb0>().to_bitvec();
			self.append(&mut num_bit_vec);
		} else if num <=4294967295 {
			//write a 32 bit number
			self.write_bit(false);
			self.write_bit(true);
			let mut num_bit_vec = num.as_u32().view_bits::<Lsb0>().to_bitvec();
			self.append(&mut num_bit_vec);
		} else {
			//write a 64 bit number
			self.write_bit(true);
			self.write_bit(true);
			let mut num_bit_vec = num.as_u64().view_bits::<Lsb0>().to_bitvec();
			self.append(&mut num_bit_vec);
		}
	}

	fn write_string(&mut self,string: String) {
		let bytes = string.into_bytes();
		self.write_number(bytes.len());
		for char in bytes {
			let mut num_bit_vec = char.as_u8().view_bits::<Lsb0>().to_bitvec();
			self.append(&mut num_bit_vec);
		}
	}

	//read
	fn read_bit(&mut self) -> bool {
		self.remove(0)
	}

	
	fn read_number(&mut self) -> usize {
		let len = match self[..2]
		.iter()
		.fold(0, |acc, elem| (acc << 1) | u8::from(*elem)) {
			0 => 8,
			1 => 16,
			2 => 32,
			3 => 64,
			_ => unreachable!("two bits cannot be greater then 3")
		};
		self.drain(..len+2)
		.skip(2)
		.rfold(0, |acc,elem| (acc << 1) | usize::from(elem))
	}
	
	fn read_string(&mut self) -> String {
		let len = self.read_number();
		let mut bytes = Vec::new();
		for _ in 0..len {
			let mut b = BitVec::<u8,Lsb0>::new();
			for _ in 0..8 {
				b.push(self.remove(0))
			}
			bytes.push(b.load::<u8>())
		};
		String::from_utf8_lossy(&bytes.into_boxed_slice()).into_owned()
	}

}

impl Node {
	//! the most basic functions for searching and accessing data in this tree	
	fn search(&self,value: &Value,path: &String) -> String {
		//! searches through the huffman tree for a value
		 
		//println!("value: {:?}, {}, s: {:?}",value,path,self);
		// check if the values match, if they do, send the string up the recursion
		let v = match value {
			Value::Keyword(kw) => {
				match self.value.clone() {
					Value::Keyword(k) =>{
						if &k == kw {path.clone()}
						else {"".to_string()}
					}
					_ => {"".to_string()}
				}
			},
			Value::Str(str) => { 
				match self.value.clone() {
					Value::Str(s) =>{ 
						if &s == str {path.clone()}
						else {"".to_string()}
					}
					_ => {"".to_string()}
				}
			},
			Value::None => {"".to_string()}
		};

		if v != "".to_string() {
			return v
		}

		// check if this node has other nodes below it, if it does check those for the value
		if self.is_leaf_node() {
			//have the left node search
			let lc = self.left_node
				.as_ref()
				.expect("left_node is *somehow* None despite being a leaf node")
				.search(&value,&(path.clone() + "0"));
			// check if we found it
			if lc == "".to_string() {
				//we did not find it down the left path, check the right
				let rc = self.left_node
					.as_ref()
					.expect("right_node is *somehow* None despite being a leaf node")
					.search(value,&(path.clone() + "1"));
				//return the right check, it will either be "" or our path
				return rc 
			} else {
				//we found the value, get the path
				return lc
			}
		}
		//this node is not a leaf and the values do not match
		return "".to_string()
	}

	fn resolve(&self,path: &String) -> Value {
		//! gets a value out of the huffman tree,
		let is_leaf = self.is_leaf_node();
		if path.len() == 0 {
			if is_leaf {
				return Value::None
			} else {
				return self.value.clone()
			}
		}
		if is_leaf {
			let mut path_clone = path.clone();
			let _ = path_clone.remove(0);
			if path.as_bytes()[0] == 48 { //check if the first letter is 0
				return self.left_node
					.as_ref()
					.expect("Somehow the left node is None despite being a leaf node")
					.resolve(&path_clone)
			} else {
				return self.right_node
					.as_ref()
					.expect("Somehow the right node is None despite being a leaf node")
					.resolve(&path_clone)
			}
		}
		return Value::None
	}

	fn is_leaf_node(&self) -> bool {
		//! just checks if it has a left node, assumming we have a right node
		self.left_node.is_some()
	}

}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("value", &self.value).field("left_node", &self.left_node).field("right_node", &self.right_node).field("weight", &self.weight).finish()
    }
}

pub fn node_into_bitvec(node: &Node,bitvec: &mut BitVec<u8>) {
	//! encodes a node into a bitvec
	if node.is_leaf_node() {
		bitvec.write_bit(false);
		node_into_bitvec(
			&node.left_node.as_ref().unwrap(),
			bitvec
		);
		node_into_bitvec(
			&node.right_node.as_ref().unwrap(),
			bitvec
		)
	} else {
		bitvec.write_bit(true);
		match &node.value {
			Value::Str(word) => {
				bitvec.write_bit(true);
				bitvec.write_string(word.clone());
			}
			Value::Keyword(kw) => {
				bitvec.write_bit(false);
				bitvec.write_number(kw.clone() as usize)
			}
			Value::None => {
				//How? Why?
			}
		}
	}
}

pub fn node_from_bitvec(bitvec: &mut BitVec<u8>) -> Node {
	//! decodes a node tree from a bitvec
	if bitvec.read_bit() {
		let v: Value = if bitvec.read_bit() {
			Value::Str(bitvec.read_string())
		} else {
			Value::Keyword(Keywords::from_usize(bitvec.read_number()))
		};
		
		return Node {
			value: v,
			left_node: None,
			right_node: None,
			weight: 0
		}
	} else {
		let left_node = node_from_bitvec(bitvec);
		let right_node = node_from_bitvec(bitvec);
		Node {
			value: Value::None,
			left_node: Some(Box::from(left_node)),
			right_node: Some(Box::from(right_node)),
			weight: 0
		}
	}
}

pub fn tokenize(words: Vec<String>) -> Vec<Value> {
	let mut output: Vec<Value> = Vec::new();
	for word in words {
		let kw = Keywords::from_string(&word.to_uppercase());
		if kw == Keywords::INVALID {
			output.push(Value::Str(word))
		} else {
			output.push(Value::Keyword(kw))
		}
	};
	output
}

pub fn stringify(values: Vec<Value>) -> Vec<String> {
	let mut output = Vec::<String>::new();
	for value in values {
		match value {
			Value::Str(string) => {output.push(string)}
			Value::Keyword(keyword) => {output.push(keyword.to_string())}
			Value::None => {}
		}
	}
	output
}

pub fn gen_counts(values: &Vec<Value>) -> BTreeMap<Value,usize> {
	let mut out: BTreeMap<Value,usize> = BTreeMap::new();
	for v in values {
		if out.contains_key(v) {
			*out.get_mut(v).unwrap() += 1;
		} else {
			out.insert(v.clone(), 1);
		}
	}
	out
}

pub fn generate_tree(counts: &BTreeMap<Value,usize>) -> Node {
	
	//Step 1 create a Vector of all the nodes
	let mut nodes: Vec<Node> = Vec::new();
	for (val,count) in counts {
		#[cfg(debug_assertions)]
		println!("[huffman::gen_tree] creating node {:?} with weight {}",val,count);
		nodes.push(
			Node {
				value: val.clone(),
				left_node: None,
				right_node: None,
				weight: count.clone()
			}
		)
	}

	//Step 2 generate the huffman tree
	while nodes.len() > 1 {
		nodes.sort_unstable_by_key(|node| node.weight);

		#[cfg(debug_assertions)]
		println!("[huffman::gen_tree] sorted {:?}",nodes);

		//take the lowest 2 nodes out of the Vec
		let left = nodes.remove(0);
		let lw = left.weight.clone(); //get the left weight
		let right = nodes.remove(0);
		let rw = right.weight.clone(); //get the right weight
		
		#[cfg(debug_assertions)]
		println!("merging {:?} and {:?}",left,right);

		let new_node = Node {
			value: Value::None,
			left_node: Some(Box::from(left)),
			right_node: Some(Box::from(right)),
			weight: lw + rw
		};

		nodes.push(new_node)
	}
	#[cfg(debug_assertions)]
	println!("[huffman::gen_tree] {:?}",nodes);
	nodes.remove(0)
}

pub fn get_paths(tree: &Node) -> BTreeMap<Value,String> {
	let mut map = BTreeMap::<Value,String>::new();
	get_paths_recursive(tree, &mut map,"".to_string());
	map
}

fn get_paths_recursive(tree: &Node, map: &mut BTreeMap<Value,String>,pth: String) {
	if tree.is_leaf_node() {
		get_paths_recursive(&tree.left_node.as_ref().unwrap(),map,pth.clone() + "0");
		get_paths_recursive(&tree.right_node.as_ref().unwrap(),map,pth + "1");
	} else {
		map.insert(tree.value.clone(), pth);
	}
}

pub fn encode_values(values: &Vec<Value>,paths: &BTreeMap<Value,String>) -> BitVec<u8> {
	let mut output = BitVec::<u8,Lsb0>::new();
	for value in values {
		let binary_string = paths.get(value).unwrap();
		
		//#[cfg(debug_assertions)]
		//println!("[huffman::encode_values] writing binary for {:?}, bits: {}",value, binary_string);
		
		for bit in binary_string.chars() {
			if bit == '0' {
				output.write_bit(false)
			} else {
				output.write_bit(true)
			}
		}
	};
	return output
}

pub fn decode_values(mut bitvec: BitVec<u8>,tree: &Node) -> Vec<Value> {
	let mut buf = "".to_string();
	let mut output: Vec<Value> = Vec::new();
	while bitvec.len() > 0 {
		if bitvec.read_bit() {
			buf += "1"
		} else { buf += "0" }
		let value = tree.resolve(&buf);
		if value != Value::None {
			output.push(value);
			buf = "".to_string()
		}
	}
	output
}

#[cfg(test)]
mod test {

	use bitvec::prelude::*;

	use crate::huffman::Keywords;

	use super::BitHelp;

	#[test]
	fn encode_decode() {
		let mut bv = BitVec::<u8,Lsb0>::new();
		bv.write_bit(true);
		bv.write_number(100);
		bv.write_string("ohno".to_string());

		println!("bv: {:?}",bv);

		println!("bit");
		let bit = bv.read_bit();
		println!("num, b={}",bit);
		let num = bv.read_number();
		println!("str, n={}",num);
		let string = bv.read_string();
		println!("s={}",string);

		println!("bit {}", bit);
		println!("num: {}",num);
		println!("string: {}", string);

		assert_eq!(bit,true);
		assert_eq!(num,100);
		assert_eq!("ohno".to_string(),string);
	}

	//#[test]
	fn keys() {
		println!("{:?}",Keywords::CALL as usize);
		assert!(false);
	}
}