mod cli;
mod errors;
use errors::*;

pub fn format_from_extension(path: &std::path::Path) -> Option<nixify::Format> {
    match path.extension()?.to_str()?.to_lowercase().as_str() {
        "toml" => Some(nixify::Format::Toml),
        "yaml" | "yml" => Some(nixify::Format::Yaml),
        "json" => Some(nixify::Format::Json),
        _ => None,
    }
}

pub fn main() -> Result<()> {
    let args = <cli::Cli as clap::Parser>::parse();
    if let Some(shell) = args.completions {
        cli::Cli::completions(shell);
        return Ok(());
    }
    let input = std::fs::read_to_string(&args.input).change_context(Error)?;
    let format = args.format.or_else(|| format_from_extension(&args.input));
    let value = if let Some(format) = format {
        nixify::parse(&input, format).change_context(Error)?
    } else {
        nixify::try_parse_all(&input).change_context(Error)?
    };
    let out = ser_nix::to_string(&value).change_context(Error)?;
    std::fs::write(&args.output, out).change_context(Error)?;

    Ok(())
}
