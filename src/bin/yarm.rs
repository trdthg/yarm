use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use clap::{Parser, ArgGroup};
use log::{info, error};
use walkdir::WalkDir;

use yarm::{Error, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
// #[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Opt {
    #[clap(short, long, help = "delete all in a folder except xxx xxx")]
    except: Option<String>,

    #[clap(help = "file or folders path that will be deleted [support regex]")]
    paths: Vec<String>,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    info!("{:?}", opt);
    if let Some(basedir) = opt.except {
        let basedir = Path::new(&basedir);
        let paths: Vec<PathBuf> = opt.paths
            .iter()
            .fold(vec![], |mut acc, path| {
                let child = WalkDir::new(path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .map(|x| x.path().to_path_buf())
                    .collect::<Vec<PathBuf>>();
                acc.extend(child);
                acc
            });
        WalkDir::new(basedir)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .for_each(|entry| {
                let path = entry.path();
                if paths.contains(&path.to_owned()) {
                    info!("skiped {:?}", path);
                    return
                }
                match fs::remove_file(path) {
                    Ok(_) => info!("deleted {:?}", path),
                    Err(e) => error!("failed to delete file {:?}: {}", path, e)
                }
            });
    } else {
        for path in opt.paths {
            WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .for_each(|entry| {
                    let entry = entry;
                    let path = entry.path();
                    match fs::remove_file(path) {
                        Ok(_) => info!("deleted {:?}", path),
                        Err(e) => error!("failed to delete file {:?}: {}", path, e)
                    }
                });
        }
    }
    Ok(())
}
