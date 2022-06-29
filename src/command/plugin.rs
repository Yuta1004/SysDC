use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[clap(name="subcommand")]
pub struct PluginCmd {
    #[clap(subcommand)]
    sub: Commands
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    /// Add a plugin
    add,

    /// Remove a plugin
    remove,

    /// Upgrade a plugin
    upgrade,

    /// Print informations of plugin
    info
}

impl PluginCmd {
    pub fn run(&self) {
        match self.sub {
            Commands::add => println!("Plugin add"),
            Commands::remove => println!("Plugin remove"),
            Commands::upgrade => println!("Plugin upgrade"),
            Commands::info => println!("Plugin info")
        }
    }
}
