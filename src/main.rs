use debugger;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

mod debugger {
    #[allow(dead_code)]
    pub struct Engine {
        path: String,
        pid: u32,
    }

    impl Engine {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_process() -> Result<(), String> {
        let debugger = debugger::Engine {
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
