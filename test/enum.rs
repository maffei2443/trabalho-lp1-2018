#[derive(Debug)]
enum IPvX {
  v4,
  v6,
}

// leve critica: pq nao ver "v4: String" ?
enum IPvX_integrated {
  v4(String),
  v6(String),
}

// enum abaixo eh definido pela biblioteca padrao
// enum Option <T> {
//   Some(T),
//   None,
// }

struct IP {
  version: IPvX,
  addr: String,
}

// fn versionIP(ip: &IPvX) -> str {

// }

fn main() {
  let four = IPvX::v4;
  let six = IPvX::v6;
  let mut homeIP = IP{ version: IPvX::v4, addr: String::from("127.0.0.1") };
  println!("{:?}", four);
  println!("{:?}", six);
  // println!("{:?}", homeIP);
  let x: Option<i32> = None;
  println!("{:?}", x);
}
