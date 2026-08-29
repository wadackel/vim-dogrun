use dogrun::highlight::{get_highlights, get_palette};
use dogrun::writer::{Writer, update_readme_fzf};
use std::env;
use std::fs::{File, create_dir_all};
use std::io;
use std::path::PathBuf;

// Parses the only supported flag (-d/--dir <path>). Not worth a clap
// dependency: this binary is an internal build tool that nobody installs,
// and dropping clap removes ten transitive crates.
fn parse_dir_arg() -> Result<Option<String>, String> {
    let mut args = env::args().skip(1);

    match args.next() {
        None => Ok(None),
        Some(flag) if flag == "-d" || flag == "--dir" => match args.next() {
            Some(dir) => Ok(Some(dir)),
            None => Err(format!("missing value for '{}'", flag)),
        },
        Some(flag) => Err(format!("unknown argument '{}'", flag)),
    }
}

fn main() -> io::Result<()> {
    let dir = parse_dir_arg().unwrap_or_else(|message| {
        eprintln!("error: {}", message);
        eprintln!("usage: dogrun [-d|--dir <output directory>]");
        std::process::exit(2);
    });

    match dir {
        Some(dir) => {
            let dir = std::path::absolute(PathBuf::from(dir))?;
            let writer = Writer::new(get_palette(), get_highlights());

            let path = File::create(dir.join("colors/dogrun.vim"))?;
            writer.write_colorscheme(io::BufWriter::new(path))?;

            let path = File::create(dir.join("autoload/lightline/colorscheme/dogrun.vim"))?;
            writer.write_lightline(io::BufWriter::new(path))?;

            let path = File::create(dir.join("autoload/clap/themes/dogrun.vim"))?;
            writer.write_clap(io::BufWriter::new(path))?;

            let wezterm_dir = dir.join("wezterm");
            create_dir_all(&wezterm_dir)?;
            let path = File::create(wezterm_dir.join("dogrun.toml"))?;
            writer.write_wezterm(io::BufWriter::new(path))?;

            // Update README.md with generated fzf colors (if it exists)
            let readme_path = dir.join("README.md");
            if readme_path.exists() {
                update_readme_fzf(&writer, &readme_path)?;
            }
        }
        None => {
            let writer = Writer::new(get_palette(), get_highlights());
            writer.write_colorscheme(io::stdout())?;
            writer.write_lightline(io::stdout())?;
            writer.write_clap(io::stdout())?;
        }
    };

    Ok(())
}
