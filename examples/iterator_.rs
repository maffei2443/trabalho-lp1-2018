fn main() {
      let v:Vec<i32> = [0;15 as usize].iter()
                        .enumerate()
                        .map( | (idx, _val )|  {
                            println!("{:?}", _val);
                            if idx & 1 == 1 {
                              2
                            } 
                            else {        
                              1
                            }
                            }).collect();
  println!("{:?}", v);
}
