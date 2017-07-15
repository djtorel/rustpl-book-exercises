#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub mod outermost {
    pub fn middle_function(){
        println!("Executing middle_function -> middle_secret_function");
        middle_secret_function();
    }

    fn middle_secret_function(){
        println!("Executing middle_secret_function -> inner_function");
        inside::inner_function();
    }

    mod inside {
        pub fn inner_function(){
            println!("Executing inner_function -> secret_function");
            secret_function();
        }

        fn secret_function(){
            println!("You have discovered my inner most secret!!");
        }
    }
}
