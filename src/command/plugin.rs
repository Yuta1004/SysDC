use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[clap(name="subcommand")]
pub struct PluginCmd {
    #[clap(subcommand)]
    sub: PluginCmdSub
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum PluginCmdSub {
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
            PluginCmdSub::add => println!("Plugin add"),
            PluginCmdSub::remove => println!("Plugin remove"),
            PluginCmdSub::upgrade => println!("Plugin upgrade"),
            PluginCmdSub::info => println!("Plugin info")
        }
    }
}
