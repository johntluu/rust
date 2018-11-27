use std::collections::HashMap;
//let mut topenv = HashMap<String, Value>::new();

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

trait ExprC {
	fn new(n: i32) -> Self;
	fn interp(&self)-> impl Value;
}

trait Value {
	fn new (n: i32) -> Self;
	fn serialize(&self)-> String;
}


impl ExprC for NumC {
	fn new(n: i32) -> Self
	{
		NumC {n: n}
	}
	fn interp(&self) -> impl Value
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
    println!("Hello World!");
}