#[derive(clap::Parser)]
pub struct CliCmd;

impl CliCmd {
    pub fn run(&self) {}

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
