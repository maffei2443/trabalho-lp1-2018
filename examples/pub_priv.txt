mod top {
  pub struct Foo {
    pub x: i32,
    y : i32,
  }
}

struct Foo {
  pub x: i32,
  y: i32,
}

struct Foo2 {
  x: i32,
  y: i32,
}

fn main() {
  let x = 9;
  let y = 99;
//  let foo = top::Foo{x, y};
  let infoo = Foo{x, y};
  let infuu = Foo2{x, y};
}
