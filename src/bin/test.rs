#[path = "../huffman.rs" ]
mod huffman;

#[path = "../parse.rs" ]
mod parse;

use bitvec::prelude::*;
use std::{fs, io::Write};

fn main() -> std::io::Result<()> {

	let content = fs::read_to_string("hello.ils").unwrap();
	let words = parse::parse(&content);
	let tokens = huffman::tokenize(words);

	let counts = huffman::gen_counts(&tokens);
	let tree = huffman::generate_tree(&counts);

	let paths = huffman::get_paths(&tree);

	//println!("{:?}",paths);
	let mut tree_bits = BitVec::<u8,Lsb0>::new();
	huffman::node_into_bitvec(&tree, &mut tree_bits);
	let encoded_bits = huffman::encode_values(&tokens, &paths);
	//println!("{:?}",bits);
	println!("{:?}",tree_bits);
	let tree_bytes = tree_bits.clone().into_vec();
	let reconstructed_tree = huffman::node_from_bitvec(&mut tree_bits);
	println!("{:?}",reconstructed_tree);
	let encoded_bytes = encoded_bits.clone().into_vec();
	
	let mut btree_file = fs::OpenOptions::new()
		.create(true)
		.write(true)
		.open("btree.bin").unwrap();
	
	//write the btree to a file
	//drop the errors into the abyss
	drop(btree_file.write_all(&tree_bytes.into_boxed_slice()));
	drop(btree_file.flush());
	drop(btree_file);

	let mut encoded_file = fs::OpenOptions::new()
		.create(true)
		.write(true)
		.open("encoded.bin").unwrap();

	//write the encoded data to a file
	//drop the errors into the abyss
	drop(encoded_file.write_all(&encoded_bytes.into_boxed_slice()));
	drop(encoded_file.flush());
	drop(encoded_file);

	let decoded = huffman::decode_values(encoded_bits, &reconstructed_tree);
	println!("{:?}",decoded);

	let words = huffman::stringify(decoded);
	let file = parse::combine(words);
	println!("{}",file);
	Ok(())
}