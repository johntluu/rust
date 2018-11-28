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

struct StringC {
    s: String
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

//struct PrimV {
//	f: fn(&Value, &Value) -> &Value
//}	

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
impl ExprC<String, StrV> for StringC {
	type Value = StrV;
	fn new(s: String) -> Self
	{
		StringC {s: s}
	}
	fn interp(&self) -> StrV
	{
		StrV::new(self.s.to_string())
	}
}

/*impl ExprC<PrimV> for AppC {
	type Value = NumV;
	fn new(n: i32) -> Self
	{
		NumC {n: n}
	}
	fn interp(&self) -> NumV
	{
		NumV::new(self.n)
	}
}*/

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


impl Value<String, StrV> for StrV {
	fn new(s: String) -> Self
	{
		StrV {s: s}
	}
	fn serialize(&self) -> String
	{
		self.s.to_string()
	}
}

//impl Value for CloV {
	//fn new(b: bool) -> Self
	//{
	//	CloV {b: b}
	//}
//	fn serialize(&self) -> String
//	{
//		"#<procedure>"
//	}
//}

//impl Value for PrimV {
	//fn new(b: bool) -> Self
	//{
	//	PrimV {b: b}
	//}
//	fn serialize(&self) -> String
//	{
//		"#<primop>"
//}


fn main() {
	let num1 = NumC {n: 1};
	let num1V = NumV {n: 1}; 
	let nv1 = num1.interp();
	assert_eq!(num1V.n, nv1.n);
	let st1 = StringC {s: "hello".to_string()};
	let st1V = StrV {s: "hello".to_string()}; 
	let sv1 = st1.interp();
	assert_eq!(st1V.s, sv1.s);
	let bo1 = BoolC {b: true};
	let bo1V = BoolV {b: true}; 
	let bv1 = bo1.interp();
	assert_eq!(bo1V.b, bv1.b);
    println!("Hello World!");

    let n1 = NumC {n: 53};
    let str1 = StringC {s: "lol".to_string()};
    let b1 = BoolC {b: true};
    let sym1 = IdC {s: "haha"};
    //let app1 = AppC {func: &NumC {n: 1}, args: vec![]};
    println!("NumC: {}", n1.n);
    println!("StringC: {}", str1.s);
    println!("BoolC: {}", b1.b);
    println!("IdC: {}", sym1.s);
    println!("{}", n1);
}