use std::env;

fn parse(value : &Vec<&str>) -> bool {
	for &c in value {
		if !c.chars().all(|x| x.is_numeric()) && !["+", "-", "*", "/"].contains(&c){
			return false;
		}
	}
	true
}

fn apply_op(stack : &mut Vec<i32>, op : fn(i32, i32)-> i32) -> bool {
	if stack.len() < 2 { return false; }
	let var : (i32, i32) = (stack.pop().expect("not enough numbers"), stack.pop().expect("not enough number"));
	stack.push(op(var.0, var.1));
	true
}

fn rpn(value : &String) {
	let oper : Vec<&str> = value.split_whitespace().collect();
	let mut stack : Vec<i32> = vec![];

	if !parse(&oper) {
		println!("Error : unexpected value.\n");
	}

	for c in oper {
		match c {
			"+" => { if !apply_op(&mut stack, |x, y| x + y) { return ;} },
			"-" => { if !apply_op(&mut stack, |x, y| y - x) { return ;} },
			"*" => { if !apply_op(&mut stack, |x, y| x * y) { return ;} },
			"/" => { if !apply_op(&mut stack, |x, y| y / x) { return ;} },
			_ => stack.push(c.parse().unwrap())
		}
	}

	if stack.len() > 1 {
		println!("wrong calcul.")
	}
	else {
		println!("result : {}", stack.last().expect("Wrong calcul."));
	}
}

fn main() {
	if env::args().count() != 2 {
		println!("./rpn <operation>");
		return ;
	}

	let args = &(env::args().last());
	match args {
		Some(value) => rpn(&(String::from(value.trim()))),
		None => {
			println!("Error.");
			return
		}
	}

}
