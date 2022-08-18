use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use serde::Serialize;
use rmp_serde::Serializer;

use crate::compiler::Compiler;
use crate::compiler::structure::SysDCSystem;
use crate::plugin::PluginManager;

#[derive(Debug)]
enum CommandError {
    NotFoundError(String),
    SyntaxError(String),
    RuntimeError(String)
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandError::NotFoundError(text) => write!(f, "{} is not found (CommandError::NotFoundError)", text),
            CommandError::SyntaxError(text) => write!(f, "{} (CommandError::SyntaxError)", text),
            CommandError::RuntimeError(text) => write!(f, "{} (CommandError::RuntimeError)", text)
        }
    }
}

#[derive(clap::Parser)]
pub struct InteractiveCmd {
    #[clap(skip=None)]
    system: Option<SysDCSystem>,

    #[clap(skip=PluginManager::new())]
    plugin_manager: PluginManager
}

impl InteractiveCmd {
    pub fn run(&mut self) {
        loop {
            match self.run_one_line() {
                Ok(do_exit) => {
                    println!("Ok\n");
                    if do_exit {
                        break
                    }
                },
                Err(e) => println!("[ERROR] {}\n", e)
            }
        }
    }

    fn run_one_line(&mut self) -> Result<bool, Box<dyn Error>> {
        print!("> ");
        io::stdout().flush().unwrap(); 

        let mut text = String::new();
        io::stdin().read_line(&mut text)?;
        let (cmd, subcmd, args) = InteractiveCmd::parse_input(text.trim().to_string())?;

        match cmd.as_str() {
            "exit" => {
                println!("Bye...");
                Ok(true)
            },
            "in" => {
                self.run_mode_in(subcmd, args)?;
                Ok(false)
            },
            "out" => {
                self.run_mode_out(subcmd, args)?;
                Ok(false)
            },
            "load" => {
                self.load_system(subcmd)?;
                Ok(false)
            }
            "save" => {
                self.save_system(subcmd)?;
                Ok(false)
            }
            _ => {
                Err(Box::new(
                    CommandError::NotFoundError(format!("Command \"{}\"", cmd))
                ))
            }
        }
    }

    fn run_mode_in(&mut self, name: String, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let plugin = match self.plugin_manager.get_type_in(&name) {
            Some(plugin) => plugin,
            None => {
                return Err(Box::new(
                    CommandError::NotFoundError(format!("Plugin \"{}\"", name))
                ));
            }
        };

        let mut compiler = Compiler::new();
        for (unit_name, program) in plugin.run(args)? {
            println!("Load: {}", unit_name);
            compiler.add_unit(unit_name, &program);
        }
        self.system = Some(compiler.generate_system());
        Ok(())
    }

    fn run_mode_out(&self, name: String, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let plugin = match self.plugin_manager.get_type_out(&name) {
            Some(plugin) => plugin,
            None => {
                return Err(Box::new(
                    CommandError::NotFoundError(format!("Plugin \"{}\"", name))
                ));
            }
        };

        match &self.system {
            Some(s) => plugin.run(args, s),
            None => Err(Box::new(
                CommandError::RuntimeError("Must run \"in\" command before run \"out\" command".to_string())
            ))
        }
    }

    fn load_system(&mut self, filepath: String) -> Result<(), Box<dyn Error>> {
        let serialized_system = fs::read(filepath+".sysdc")?;
        self.system = Some(rmp_serde::from_slice::<SysDCSystem>(&serialized_system[..])?);
        Ok(())
    }

    fn save_system(&self, filepath: String) -> Result<(), Box<dyn Error>> {
        let serialized_system = match &self.system {
            Some(s) => {
                let mut buf = vec!();
                s.serialize(&mut Serializer::new(&mut buf))?;
                buf
            },
            None => return Err(Box::new(
                CommandError::RuntimeError("Must run \"in\" command before run \"save\" command".to_string())
            ))
        };

        let mut f = File::create(&(filepath+".sysdc"))?;
        f.write_all(&serialized_system)?;
        f.flush()?;
        Ok(())
    }

    fn parse_input(text: String) -> Result<(String, String, Vec<String>), Box<dyn Error>> {
        let splitted_text = text.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        match splitted_text.len() {
            1 => {
                if splitted_text[0].len() == 0 {
                    return Err(Box::new(
                        CommandError::SyntaxError("Usage: in/out/save <name> <args>".to_string())
                    ));
                }
                Ok((splitted_text[0].clone(), "".to_string(), vec!()))
            },
            2 => Ok((splitted_text[0].clone(), splitted_text[1].clone(), vec!())),
            _ => Ok((splitted_text[0].clone(), splitted_text[1].clone(), splitted_text[2..].to_vec()))
        }
    } 
}

#[cfg(test)]
mod test {
    use super::InteractiveCmd;

    #[test]
    fn test_parse_input() {
        assert!(InteractiveCmd::parse_input("".to_string()).is_err());

        match InteractiveCmd::parse_input("aaa".to_string()) {
            Ok((cmd, subcmd, args)) => {
                let empty_string_vec: Vec<String> = vec!();
                assert_eq!(cmd, "aaa");
                assert_eq!(subcmd, "");
                assert_eq!(args, empty_string_vec);
            },
            Err(e) => panic!("{}", e)
        }

        match InteractiveCmd::parse_input("aaa bbb".to_string()) {
            Ok((cmd, subcmd, args)) => {
                let empty_string_vec: Vec<String> = vec!();
                assert_eq!(cmd, "aaa");
                assert_eq!(subcmd, "bbb");
                assert_eq!(args, empty_string_vec);
            },
            Err(e) => panic!("{}", e)
        } 

        match InteractiveCmd::parse_input("aaa bbb ccc".to_string()) {
            Ok((cmd, subcmd, args)) => {
                assert_eq!(cmd, "aaa");
                assert_eq!(subcmd, "bbb");
                assert_eq!(args, vec!("ccc".to_string()));
            },
            Err(e) => panic!("{}", e)
        }
    }
}
