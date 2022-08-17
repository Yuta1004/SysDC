use std::io;
use std::io::Write;
use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use crate::compiler::Compiler;
use crate::compiler::structure::SysDCSystem;
use crate::plugin::PluginManager;

#[derive(Debug)]
enum CommandError {
    SyntaxError(String),
    RuntimeError(String)
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandError::SyntaxError(text) => write!(f, "{} (CommandError::SyntaxError)", text),
            CommandError::RuntimeError(text) => write!(f, "{} (CommandError::RuntimeError)", text)
        }
    }
}

#[derive(clap::Parser)]
pub struct InteractiveCmd;

impl InteractiveCmd {
    pub fn run(&self) {
        let mut system: Option<SysDCSystem> = None;
        let plugin_manager = PluginManager::new();
        loop {
            match InteractiveCmd::run_one_line(&plugin_manager, &system) {
                Ok((do_exit, _system)) => {
                    if _system.is_some() {
                        system = _system;
                    }
                    println!("Ok\n");
                    if do_exit {
                        break
                    }
                },
                Err(e) => println!("[ERROR] {}\n", e)
            }
        }
    }

    fn run_one_line(plugin_manager: &PluginManager, system: &Option<SysDCSystem>) -> Result<(bool, Option<SysDCSystem>), Box<dyn Error>> {
        print!("> ");
        io::stdout().flush().unwrap(); 

        let mut text = String::new();
        io::stdin().read_line(&mut text)?;
        let (cmd, subcmd, args) = InteractiveCmd::parse_input(text.trim().to_string())?;

        match cmd.as_str() {
            "exit" => Ok((true, None)),
            "in" => {
                let _system = InteractiveCmd::run_mode_in(plugin_manager, subcmd, args)?;
                Ok((false, Some(_system)))
            },
            "out" => {
                match system {
                    Some(s) => {
                        InteractiveCmd::run_mode_out(plugin_manager, subcmd, args, s)?;
                        Ok((false, None))
                    },
                    None => Err(Box::new(
                        CommandError::RuntimeError("Must run \"in\" before run \"out\"".to_string())
                    ))
                }
            },
            _ => {
                Err(Box::new(
                    CommandError::RuntimeError(format!("\"{}\" not found", cmd))
                ))
            }
        }
    }

    fn run_mode_in(plugin_manager: &PluginManager, name: String, args: Vec<String>) -> Result<SysDCSystem, Box<dyn Error>> {
        let plugin = match plugin_manager.get_type_in(&name) {
            Some(plugin) => plugin,
            None => {
                return Err(Box::new(
                    CommandError::RuntimeError(format!("\"{}\" not found", name))
                ));
            }
        };

        let mut compiler = Compiler::new();
        for (unit_name, program) in plugin.run(args)? {
            compiler.add_unit(unit_name, &program);
        }
        Ok(compiler.generate_system())
    }

    fn run_mode_out(plugin_manager: &PluginManager, name: String, args: Vec<String>, system: &SysDCSystem) -> Result<(), Box<dyn Error>> {
        let plugin = match plugin_manager.get_type_out(&name) {
            Some(plugin) => plugin,
            None => {
                return Err(Box::new(
                    CommandError::RuntimeError(format!("\"{}\" not found", name))
                ));
            }
        };
        plugin.run(args, system)
    }

    fn parse_input(text: String) -> Result<(String, String, Vec<String>), Box<dyn Error>> {
        let splitted_text = text.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        match splitted_text.len() {
            1 => {
                if splitted_text[0].len() == 0 {
                    return Err(Box::new(
                        CommandError::SyntaxError("Usage: in/out <name> <args>".to_string())
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
