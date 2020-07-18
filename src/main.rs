use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

#[cfg(test)]
mod tests {
    use super::*; // This gives us access to the outer scope.

    #[test]
    fn create_process() -> Result<(), String> {
        Ok(())

    }
}
