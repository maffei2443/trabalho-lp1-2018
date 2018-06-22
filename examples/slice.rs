fn main(){
  let a = [0,1,2,3];
  let mut sli = &a[1..3];
  println!("{}", sli.len());
  sli = &[];
  println!("sizeof(a) == {}", a.len());
  println!("sizeof(sli) == {}", sli.len());
}
