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
        let debugger = Debugger {
            path: String::from(""),
            pid: 12,
        };
        if debugger.pid == 0 {
            panic!(
                "thing.pid cannot be 0, was {0}, path was {1}",
                debugger.pid, debugger.path
            );
        };
        Ok(())
    }
}

#[allow(dead_code)]
pub struct Debugger {
    path: String,
    pid: u32,
}

impl Debugger {}
