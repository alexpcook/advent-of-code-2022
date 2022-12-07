use std::collections::HashMap;

use anyhow::bail;

pub fn main(input: String) -> anyhow::Result<()> {
    let mut filesystem: HashMap<String, u64> = HashMap::from([("/".to_string(), 0)]);

    let mut current_dir = vec![];

    for line in input.lines().filter(|line| !line.is_empty()) {
        let first_char = line.chars().next().unwrap();

        match first_char {
            '$' => {
                let raw_cmd: Vec<_> = line.split_whitespace().collect();
                let cmd = *raw_cmd.get(1).unwrap();
                assert!(cmd == "cd" || cmd == "ls");

                if cmd == "cd" {
                    let dst = *raw_cmd.get(2).unwrap();
                    assert!(dst == "/" || dst == ".." || dst.is_ascii());

                    match dst {
                        "/" => {
                            current_dir.clear();
                        }
                        ".." => {
                            current_dir.pop();
                        }
                        dir => {
                            current_dir.push(dir);
                        }
                    }
                } else {
                }
            }
            'd' => {}
            '0'..='9' => {
                let size: u64 = line.split_whitespace().next().unwrap().parse().unwrap();

                let mut path = "/".to_string();

                *filesystem.entry(path.clone()).or_default() += size;

                for dir in &current_dir {
                    if !path.ends_with('/') {
                        path.push('/');
                    }
                    path.push_str(dir);

                    *filesystem.entry(path.clone()).or_default() += size;
                }
            }
            _ => bail!("unknown first character"),
        }
    }

    let total_size: u64 = filesystem.values().filter(|&&size| size <= 100000).sum();

    println!("total size of directories: {total_size}");

    let used_space: u64 = *filesystem.get("/").unwrap();
    let total_disk_space = 70_000_000;
    let needed_space = 30_000_000;
    let free_space = total_disk_space - used_space;
    let need_to_deleted = needed_space - free_space;
    let size_of_directory_to_delete = filesystem
        .values()
        .filter(|&&size| size >= need_to_deleted)
        .min()
        .unwrap();

    println!("size of directory to delete: {size_of_directory_to_delete}");

    Ok(())
}
