use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::fmt;
use std::fmt::{ Display, Formatter };
use std::process;
use std::error::Error;

use serde::Serialize;
use rmp_serde::Serializer;

use crate::parser::Parser;
use crate::parser::structure::SysDCSystem;
use crate::plugin::PluginManager;

#[derive(Debug)]
enum CommandError {
    NotFound(String),
    Syntax,
    SystemIsNotSetted
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandError::NotFound(command) => write!(f, "Command \"{}\" is not found", command),
            CommandError::Syntax => write!(f, "Usage: in/out/load/save <name> <args>"),
            CommandError::SystemIsNotSetted => write!(f, "Must run \"in\" command before run command")
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
                Ok(_) => println!(),
                Err(e) => println!("[ERROR] {}\n", e)
            }
        }
    }

    fn run_one_line(&mut self) -> Result<(), Box<dyn Error>> {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut text = String::new();
        io::stdin().read_line(&mut text)?;
        let (cmd, subcmd, args) = InteractiveCmd::parse_input(text.trim().to_string())?;

        match cmd.as_str() {
            "exit" => self.exit_interactive_mode(),
            "in" => self.run_mode_in(subcmd, args)?,
            "out" => self.run_mode_out(subcmd, args)?,
            "load" => self.load_system(subcmd)?,
            "save" => self.save_system(subcmd)?,
            _ => return Err(Box::new(CommandError::NotFound(cmd)))
        }
        Ok(())
    }

    fn exit_interactive_mode(&mut self) {
        println!("Bye..");
        process::exit(0);
    }

    fn run_mode_in(&mut self, name: String, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let plugin = self.plugin_manager.get_type_in(&name)?;
        let mut parser = Parser::new();
        for (filename, program) in plugin.run(args)? {
            println!("Loading: {}", filename);
            parser.parse(program)?;
        }
        self.system = Some(parser.check()?);
        Ok(())
    }

    fn run_mode_out(&self, name: String, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let plugin = self.plugin_manager.get_type_out(&name)?;
        match &self.system {
            Some(s) => plugin.run(args, s),
            None => Err(Box::new(CommandError::SystemIsNotSetted))
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
            None => return Err(Box::new(CommandError::SystemIsNotSetted))
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
                    return Err(Box::new(CommandError::Syntax));
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
