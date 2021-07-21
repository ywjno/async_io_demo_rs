use humansize::{file_size_opts, FileSize};
use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("input path PLZ.")
    }
    // args[1]: 统计的目录名
    // println!("{}", args[1]);
    let space_size = cal_space_size(PathBuf::from(args[1].to_string()))?;
    let usages = space_size.file_size(file_size_opts::CONVENTIONAL)?;
    let work_path = dunce::canonicalize(PathBuf::from(args[1].to_string()))?;
    println!("{:?} space size: {}", work_path, usages);
    Ok(())
}

fn cal_space_size(path: PathBuf) -> std::io::Result<u64> {
    let mut paths = vec![path];
    let mut res_size = 0;
    while let Some(path) = paths.pop() {
        let meta = fs::symlink_metadata(&path)?;
        let file_type = meta.file_type();
        if file_type.is_dir() {
            let entries = fs::read_dir(path)?;
            for entry in entries {
                paths.push(entry?.path());
            }
        }
        if file_type.is_file() {
            res_size += meta.len();
        }
    }
    Ok(res_size)
}
