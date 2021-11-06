use config::{config_path, make_new_config, read_config, Language};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use structopt::clap;

use crate::config::Config;

mod config;

fn main() {
    let opt = Opt::from_args();

    let config_path = config_path();
    if !config_path.exists() {
        make_new_config(config_path.clone());
    }
    let config = read_config(config_path.clone());

    make_new_folder(opt, config);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "kyopro folder")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    /// Folder name
    folder_name: String,

    /// Programing language
    lang: String,

    /// File names
    srcs: Vec<String>,
}

fn make_new_folder(opt: Opt, config: Config) {
    let dir = Path::new(".");
    let map = config
        .languages
        .iter()
        .map(|e| (&e.language, e))
        .collect::<HashMap<_, _>>();
    let Language {
        language: _,
        extention,
        template,
    } = map.get(&opt.lang).expect("Failed to get toml item.");

    fs::create_dir(dir.join(&opt.folder_name)).expect("Failed to create directory.");

    for src in opt.srcs.iter() {
        fs::write(
            dir.join(&opt.folder_name)
                .join(src)
                .with_extension(extention),
            template.as_ref().unwrap_or(&"".to_string()),
        )
        .expect("Failed to write template.");
    }
}
