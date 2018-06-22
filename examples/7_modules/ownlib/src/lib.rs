// #[cfg(test)]
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function(); // fail to compile because 'middle_secret_function' is private
    outermost::inside::inner_function(); // fail, because 'inside' is private
    outermost::inside::secret_function(); // fail to compile because 'secret_function' is private
}