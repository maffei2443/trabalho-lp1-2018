#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod summarize {
	pub trait Summary {
	    fn summarize(&self) -> String;
	}

	impl Summary for i32 {
		fn summarize(&self) -> String{
			println!("{:?}", self.to_string());
			self.to_string()
		}
	}
}