pub struct Options {
    pub take: String,
    pub mounts: Vec<String>,
    pub threads: i32,
}

impl Options {
    pub fn new(args: &[String]) -> Options {
        if args.len() == 1 {
            Options::help();
        }

        let mut result = Options {
            take: String::new(),
            mounts: Vec::new(),
            threads: 1,
        };

        let mut i = 1;

        while i < args.len() {
            let command = &args[i][1..];

            let mut j = i + 1;
            loop {
                if j < args.len() && Options::is_parameter(&args[j]) {
                    result.handle_all(command, &args[j]);
                } else {
                    if j == i + 1 {
                        result.handle_all(command, "");
                    }

                    break;
                }

                j += 1;
            }

            i = j;
        }

        result
    }

    fn handle_all(&mut self, command: &str, parameter: &str) {
        if command[..1] == "-".to_string() {
            self.handle(&command[..1], parameter);
            return;
        }

        for i in 0..command.len() {
            if !self.handle(&command[i..i + 1], parameter) {
                return;
            }
        }
    }

    fn handle(&mut self, command: &str, parameter: &str) -> bool {
        if "help".to_string() == command || "h".to_string() == command {
            Options::help();
        } else if "input" == command || "i" == command {
            self.take = parameter.to_string();
        } else if "mount" == command || "m" == command {
            self.mounts.push(parameter.to_string());
        } else if "threads" == command || "t" == command {
            if let Ok(t) = parameter.parse::<i32>() {
                self.threads = t;
            }
        } else {
            // option does not exist
        }

        true
    }

    fn is_parameter(text: &str) -> bool {
        if text.len() <= 1 {
            return true;
        }

        if text[0..1] == "-".to_string() {
            if text[1..2] == "-".to_string() {
                return false;
            }

            return text[1..].chars().all(char::is_numeric);
        }

        true
    }

    fn help() {
        let text = r#"(bud is a global illumination renderer experiment
Usage:
  sprout [OPTION...]

  -h, --help                    Print help.
  -i, --input json file/string  Path of the take file to render or json
                                string describing the take.
  -m, --mount directory path    Specifies a mount point for the data
                                directory. The default value is "../data/"
  -t, --threads integer         Specifies the number of threads used by
                                sprout. 0 creates one thread for each logical CPU.
                                -x creates a number of threads equal to the
                                number of logical CPUs minus x. The default
                                value is 0.
  -p, --progressive             Starts sprout in progressive mode.
      --no-textures             Disables loading of all textures.
  -v, --verbose                 Enables verbose logging.)"#;

        println!("{}", text);
    }
}
