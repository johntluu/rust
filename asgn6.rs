use std::collections::HashMap;
use std::fmt;

//let mut topenv = HashMap<String, Value>::new();
struct Env {
	hm: HashMap<String, NumV>
}

// ExprC
trait ExprC<T, RHS=Self> {
	type Value;
	fn new(n: T) -> Self;
	fn interp(&self) -> Self::Value;
}

struct NumC {
    n: i32
}

struct IdC<'a> {
    s: &'a str
}

struct BoolC {
    b: bool
}

struct StringC<'a> {
    s: &'a str
}

// struct AppC<'a> {
//     func: &'a ExprC,
//     args: Vec<&'a ExprC>
// }

// Values
trait Value<T, RHS=Self> {
	fn new(n: T) -> Self;
	fn serialize(&self)-> String;
}

struct NumV {
	n: i32
}

struct BoolV {
    b: bool
}

struct StrV {
	s: String
}

struct CloV {
	
}	

struct PrimV {
	
}	

// Method implementations
impl ExprC<i32, NumV> for NumC {
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

impl ExprC<bool, BoolV> for BoolC {
	type Value = BoolV;
	fn new(b: bool) -> Self
	{
		BoolC {b: b}
	}
	fn interp(&self) -> BoolV
	{
		BoolV::new(self.b)
	}
}

impl Value<i32, NumV> for NumV {
	fn new(n: i32) -> Self
	{
		NumV {n: n}
	}
	fn serialize(&self) -> String
	{
		self.n.to_string()
	}
}

impl Value<bool, BoolV> for BoolV {
	fn new(b: bool) -> Self
	{
		BoolV {b: b}
	}
	fn serialize(&self) -> String
	{
		self.b.to_string()
	}
}

// print for NumC
impl fmt::Display for NumC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.n)
    }
}

fn main() {
	let num1 = NumC {n: 1};
	let num1V = NumV {n: 1}; 
	let nv1 = num1.interp();
	assert_eq!(num1V.n, nv1.n);
    println!("Hello World!");

    let n1 = NumC {n: 53};
    let str1 = StringC {s: "lol"};
    let b1 = BoolC {b: true};
    let sym1 = IdC {s: "haha"};
    //let app1 = AppC {func: &NumC {n: 1}, args: vec![]};
    println!("NumC: {}", n1.n);
    println!("StringC: {}", str1.s);
    println!("BoolC: {}", b1.b);
    println!("IdC: {}", sym1.s);
    println!("{}", n1);
}