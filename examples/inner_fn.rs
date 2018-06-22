fn foo() {
  println!("outline FOO");
}

fn main() {
  let f = foo;
  {
	  let g = foo;
	  g();
	  fn foo() {
	    println!("first fooo!");
	  }
	  // the name `foo` is defined multiple times
	  // fn foo() {
	  //   println!("second fooo!");
	  // }	  
  	g();
  }
  f();
  // Tipo são inferidos após a primeira invocação da closure
  let f = |x,y| x+y;
  println!("{}", f(3,44));
  foo();
}
