#[derive(Debug)]
struct Cacher<T>
	where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher