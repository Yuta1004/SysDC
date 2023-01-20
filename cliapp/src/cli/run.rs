use clap::Parser;

#[derive(Parser)]
pub struct RunCmd {
    #[clap(short, long, default_value = "out.sysdc")]
    input: String,
}

impl RunCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("{}", self.input);
        sysdc_tool_runner::exec()
    }
}
