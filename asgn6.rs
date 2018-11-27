use std::collections::HashMap;
use std::fmt;

//let mut topenv = HashMap<String, Value>::new();
struct Env {
	hm: HashMap<String, NumV>
}

// ExprC
trait ExprC<RHS=Self> {
	type Value;
	fn new(n: i32) -> Self;
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
trait Value<RHS=Self> {
	fn new(n: i32) -> Self;
	fn serialize(&self)-> String;
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

// Method implementations
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




