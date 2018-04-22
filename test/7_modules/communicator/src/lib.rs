// #[cfg(test)]

pub mod client;

pub mod network;


#[cfg(test)]
mod test {
    // super: move u one  module in the hierarchy from the current module
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}