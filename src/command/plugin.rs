#[derive(clap::Parser)]
pub struct PluginCmd;

impl PluginCmd {
    pub fn run(&self) {
        println!("Plugin Command");
    }
}
