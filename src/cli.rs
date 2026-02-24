#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(short, long, default_value = "/dev/stdin")]
    pub input: std::path::PathBuf,
    #[clap(short, long, default_value = "/dev/stdout")]
    pub output: std::path::PathBuf,
    #[clap(short, long)]
    pub format: Option<nixify::Format>,
    #[clap(long, value_enum)]
    pub completions: Option<clap_complete::Shell>,
}

impl Cli {
    pub fn completions(shell: clap_complete::Shell) {
        let mut command = <Cli as clap::CommandFactory>::command();
        clap_complete::generate(
            shell,
            &mut command,
            env!("CARGO_BIN_NAME"),
            &mut std::io::stdout(),
        );
    }
}
