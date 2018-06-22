pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for i32 {
	fn summarize(&self) -> String{
		println!("{:?}", self.to_string());
		self.to_string()
	}
}