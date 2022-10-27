pub mod client;
pub mod network;

pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        use super::middle_secret_function;

        pub fn inner_function() {
            middle_secret_function()
        }

        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
