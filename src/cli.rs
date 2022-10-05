pub struct Arguments {
    pub command: String,
    pub flags: Vec<ArgumentFlag>
}

pub struct ArgumentFlag {
    pub name: String,
    pub value: String,
}

impl Arguments {
    pub fn new() -> Result<Arguments, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let command = args[1].clone();
        let mut flags = Vec::new();

        let mut skip_next = false;
        for (_, arg) in args.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if arg.starts_with("--") {
                let parts: Vec<&str> = arg.splitn(2, "=").collect();
                let name = parts[0].replace("--", "");
                let value = if parts.len() > 1 { parts[1].to_string() } else { String::new() };

                flags.push(ArgumentFlag { name, value });
            }
        }

        Ok(Arguments { command, flags })
    }

    pub fn get_flag(&self, name: &str) -> Option<&ArgumentFlag> {
        for flag in &self.flags {
            if flag.name == name {
                return Some(flag);
            }
        }

        None
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn print(&self) {
        println!("Command: {}", self.command);
        println!("Flags:");
        for flag in &self.flags {
            println!("  {} = {}", flag.name, flag.value);
        }
    }
}