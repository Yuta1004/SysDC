use std::io;
use std::io::Write;

use crate::compiler::Compiler;
use crate::compiler::structure::SysDCSystem;
use crate::plugin::PluginManager;

#[derive(clap::Parser)]
pub struct CliCmd;

impl CliCmd {
    pub fn run(&self) {
        let mut system = SysDCSystem::new();
        let plugin_manager = PluginManager::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap(); 

            let mut text = String::new();
            if let Err(e) = io::stdin().read_line(&mut text) {
                println!("UnknownError: {}\n", e);
                continue;
            }
            let text = match text.trim() {
                "exit" => break,
                text => text.to_string()
            };

            let (cmd, name, args) = match CliCmd::parse_cli_text(text) {
                Some(result) => result,
                None => {
                    println!("SyntaxError: Usage => in/out <name> <args>\n");
                    continue;
                }
            };

            match cmd.as_str() {
                "in" => {
                    if let Some(_system) = CliCmd::run_mode_in(&plugin_manager, name, args) {
                        system = _system;
                        println!("OK\n");
                    }
                },
                "out" => {
                    CliCmd::run_mode_out(&plugin_manager, name, args, &system);
                    println!("OK\n");
                },
                _ => {
                    println!("SyntaxError: Usage => in/out {} <args>\n", cmd);
                    continue;
                }
            }
        }
        println!("Bye...");
    }
 
    fn run_mode_in(plugin_manager: &PluginManager, name: String, args: Vec<String>) -> Option<SysDCSystem> {
        let plugin = match plugin_manager.get_type_in(&name) {
            Some(plugin) => plugin,
            None => {
                println!("PluginError: {} not found\n", name);
                return None;
            }
        };

        let mut compiler = Compiler::new();
        for (unit_name, program) in plugin.run(args) {
            compiler.add_unit(&unit_name, &program);
        }
        Some(compiler.generate_system())
    }

    fn run_mode_out(plugin_manager: &PluginManager, name: String, args: Vec<String>, system: &SysDCSystem) {
        let plugin = match plugin_manager.get_type_out(&name) {
            Some(plugin) => plugin,
            None => {
                println!("PluginError: {} not found\n", name);
                return;
            }
        };
        plugin.run(args, system);
    }

    fn parse_cli_text(text: String) -> Option<(String, String, Vec<String>)> {
        let splitted_text = text.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        match splitted_text.len() {
            0 | 1 => None,
            2 => {
                if splitted_text[1].len() == 0 {
                    return None;
                }
                Some((splitted_text[0].clone(), splitted_text[1].clone(), vec!()))
            }
            _ => Some((splitted_text[0].clone(), splitted_text[1].clone(), splitted_text[2..].to_vec()))
        }
    } 
}

#[cfg(test)]
mod test {
    use super::CliCmd;

    #[test]
    fn test_parse_cli_text() {
        assert!(CliCmd::parse_cli_text("".to_string()).is_none());
        assert!(CliCmd::parse_cli_text("aaa".to_string()).is_none());

        match CliCmd::parse_cli_text("aaa bbb".to_string()) {
            Some((mode, name, args)) => {
                let empty_string_vec: Vec<String> = vec!();
                assert_eq!(mode, "aaa");
                assert_eq!(name, "bbb");
                assert_eq!(args, empty_string_vec);
            },
            None => assert!(false)
        } 

        match CliCmd::parse_cli_text("aaa bbb ccc".to_string()) {
            Some((mode, name, args)) => {
                assert_eq!(mode, "aaa");
                assert_eq!(name, "bbb");
                assert_eq!(args, vec!("ccc".to_string()));
            },
            None => assert!(false)
        }
    }
}
