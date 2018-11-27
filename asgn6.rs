use std::collections::HashMap;
//let mut topenv = HashMap<String, Value>::new();
//use std::any::Any;
struct NumC {
    n: i32
}

struct IdC {
    s: String
}

struct BoolC {
    b: bool
}

struct NumV {
	n: i32
}

struct StrV {
	s: String
}

struct CloV {
	
}	

struct PrimV {
	
}	

struct Env{
	hm: HashMap<String, NumV>
}

trait Value<RHS=Self> {
	//type String;
	fn new(n: i32) -> Self;
	fn serialize(&self)-> String;
}

trait ExprC<RHS=Self> {
	type Value;
	fn new(n: i32) -> Self;
	fn interp(&self) -> Self::Value;
}


impl ExprC<NumV> for NumC {
	type Value = NumV;
	fn new(n: i32) -> Self
	{
		NumC {n: n}
	}
	fn interp(&self) -> NumV
	{
		NumV::new(self.n)
	}
}

impl Value for NumV {
	fn new(n: i32) -> Self
	{
		NumV {n: n}
	}
	fn serialize(&self) -> String
	{
		self.n.to_string()
	}
}

fn main() {
	let num1 = NumC {n: 1};
	let num1V = NumV {n: 1}; 
	let nv1 = num1.interp();
	assert_eq!(num1V.n, nv1);
    println!("Hello World!");
}