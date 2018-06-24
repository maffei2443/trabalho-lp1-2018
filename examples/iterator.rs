fn main() {
  let v = vec![0; 10];
  let vi:Vec<i32> = v.iter().enumerate().
                                map(|(idx, val)| {
                                    if idx%3 == 0 {
                                        0
                                    }
                                    else {
                                        125
                                    }
                                    }).collect();
  println!("{:?}", vi);
  println!("{:?}", v);
}
