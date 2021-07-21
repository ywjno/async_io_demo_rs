use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use humansize::{file_size_opts, FileSize};
use std::fs::Metadata;
use std::{env, error::Error, path::PathBuf};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("input path PLZ.")
    }
    // args[1]: 目录名
    // println!("{}", args[1]);
    let space_size = cal_space_size(PathBuf::from(args[1].to_string())).await?;
    let usages = space_size.file_size(file_size_opts::CONVENTIONAL)?;
    let work_path = dunce::canonicalize(PathBuf::from(args[1].to_string()))?;
    println!("{:?} space size: {}", work_path, usages);
    Ok(())
}

async fn cal_space_size(path: PathBuf) -> std::io::Result<u64> {
    let mut queue = FuturesUnordered::new();
    queue.push(for_path(path));
    let mut res_size = 0;
    while let Some((path, meta)) = queue.try_next().await? {
        let file_type = meta.file_type();
        if file_type.is_dir() {
            let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                queue.push(for_path(entry.path()));
            }
        }
        if file_type.is_file() {
            res_size += meta.len();
        }
    }
    Ok(res_size)
}

async fn for_path(path: PathBuf) -> std::io::Result<(PathBuf, Metadata)> {
    let meta = fs::symlink_metadata(&path).await?;
    Ok((path, meta))
}

// 错误的异步写法
// async fn cal_space_size(path: PathBuf) -> std::io::Result<u64> {
//     let mut paths = vec![path];
//     let mut res_size = 0;
//     while let Some(path) = paths.pop() {
//         let meta = fs::symlink_metadata(&path).await?;
//         let file_type = meta.file_type();
//         if file_type.is_dir() {
//             let mut entries = fs::read_dir(path).await?;
//             while let Some(entry) = entries.next_entry().await? {
//                 paths.push(entry.path());
//             }
//         }
//         if file_type.is_file() {
//             res_size += meta.len();
//         }
//     }
//     Ok(res_size)
// }
