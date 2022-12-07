use std::collections::HashMap;
use std::fmt;

use anyhow::{anyhow, bail};

/// Total disk space.
const TOTAL_DISK_SPACE: u64 = 70_000_000;

/// Needed disk space.
const NEEDED_DISK_SPACE: u64 = 30_000_000;

pub fn main(input: String) -> anyhow::Result<()> {
    let mut filesystem: HashMap<String, u64> = HashMap::new();
    let mut current_dir = Directory::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        match line.split_whitespace().next() {
            Some("$") => {
                let command = Command::try_from(line).map_err(|e| anyhow!("{e}"))?;

                match command {
                    Command::Cd { location } => match location {
                        Location::Root => current_dir.clear(),
                        Location::Parent => {
                            current_dir.pop_path();
                        }
                        Location::Directory(dir) => current_dir.push_path(dir),
                    },
                    Command::Ls => {}
                }
            }
            Some(size) if size.chars().all(|c| c.is_ascii_digit()) => {
                let size: u64 = size.parse()?;

                for dir in &current_dir {
                    *filesystem.entry(dir).or_default() += size;
                }
            }
            Some("dir") => {}
            _ => bail!("failed to parse line of input: {line}"),
        }
    }

    let part1: u64 = filesystem.values().filter(|&&size| size <= 100_000).sum();
    log::info!("total size of directories <= 100000: {part1}");

    let used_space: u64 = *filesystem
        .get("/")
        .ok_or_else(|| anyhow!("failed to get filesystem used space"))?;
    let free_space = TOTAL_DISK_SPACE - used_space;
    let space_to_delete = NEEDED_DISK_SPACE - free_space;

    let part2 = filesystem
        .values()
        .filter(|&&size| size >= space_to_delete)
        .min()
        .ok_or_else(|| anyhow!("failed to find smallest directory to delete"))?;
    log::info!("size of directory to delete: {part2}");

    Ok(())
}

/// A location in the filesystem to navigate to.
#[derive(Debug)]
enum Location {
    Root,
    Parent,
    Directory(String),
}

/// A command to run on the operating system.
#[derive(Debug)]
enum Command {
    Cd { location: Location },
    Ls,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pieces: Vec<_> = value.split_whitespace().skip(1).take(2).collect();
        match pieces.first() {
            None => Err("failed to get command"),
            Some(&"ls") => Ok(Command::Ls),
            Some(&"cd") => {
                let location = match pieces.get(1) {
                    None => return Err("failed to get location"),
                    Some(&"/") => Location::Root,
                    Some(&"..") => Location::Parent,
                    Some(loc) => Location::Directory(loc.to_string()),
                };
                Ok(Command::Cd { location })
            }
            Some(_) => Err("unknown command"),
        }
    }
}

/// A directory on the filesystem.
#[derive(Debug, Default)]
struct Directory {
    path: Vec<String>,
}

impl Directory {
    /// Create a new, empty `Directory`.
    fn new() -> Directory {
        Directory::default()
    }

    /// Push `path` into directory.
    fn push_path(&mut self, path: impl Into<String>) {
        self.path.push(path.into());
    }

    /// Pop the end of the path from directory. Returns `None` if we are at the filesystem root.
    fn pop_path(&mut self) -> Option<String> {
        self.path.pop()
    }

    /// Clears all elements from the directory path, returning to the filesystem root.
    fn clear(&mut self) {
        self.path.clear();
    }
}

impl<T> From<Vec<T>> for Directory
where
    T: Into<String>,
{
    fn from(value: Vec<T>) -> Self {
        Directory {
            path: value.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Directory
where
    T: Into<String>,
{
    fn from(value: [T; N]) -> Self {
        Directory {
            path: value.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.path.join("/"))
    }
}

/// An iterator for traversing a directory from the most specific path back to the filesystem root.
#[derive(Debug)]
struct DirectoryHierarchy<'a> {
    directory: &'a Directory,
    state: Option<usize>,
}

impl<'a> Iterator for DirectoryHierarchy<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(state) = self.state else {
            return None;
        };

        let next = Some(format!(
            "/{}",
            self.directory
                .path
                .get(..state)
                .unwrap_or_default()
                .join("/"),
        ));

        self.state = if state == 0 { None } else { Some(state - 1) };

        next
    }
}

impl<'a> IntoIterator for &'a Directory {
    type Item = <DirectoryHierarchy<'a> as Iterator>::Item;
    type IntoIter = DirectoryHierarchy<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DirectoryHierarchy {
            directory: self,
            state: Some(self.path.len()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_new() {
        let dir = Directory::new();
        assert!(dir.path.is_empty());
    }

    #[test]
    fn directory_push_pop_clear() {
        let mut dir = Directory::new();
        dir.push_path("a");

        assert_eq!(dir.path.len(), 1);
        assert_eq!(dir.path.get(0).unwrap(), "a");

        dir.push_path("b");
        assert_eq!(dir.path.len(), 2);
        assert_eq!(dir.path.get(1).unwrap(), "b");

        assert_eq!(dir.pop_path().unwrap(), "b");
        assert_eq!(dir.pop_path().unwrap(), "a");
        assert_eq!(dir.pop_path(), None);

        let mut dir = Directory::from(["x", "y", "z"]);
        dir.clear();
        assert_eq!(dir.path.len(), 0);
    }

    #[test]
    fn directory_from() {
        let dir = Directory::from(vec!["a", "b"]);
        assert_eq!(dir.path.len(), 2);
        assert_eq!(dir.path.get(0).unwrap(), "a");
        assert_eq!(dir.path.get(1).unwrap(), "b");

        let dir = Directory::from(["t".to_string(), "u".to_string(), "v".to_string()]);
        assert_eq!(dir.path.len(), 3);
        assert_eq!(dir.path.get(0).unwrap(), "t");
        assert_eq!(dir.path.get(1).unwrap(), "u");
        assert_eq!(dir.path.get(2).unwrap(), "v");
    }

    #[test]
    fn directory_display() {
        let mut dir = Directory::default();
        assert_eq!(dir.to_string(), "/");

        dir.path.push("a".to_string());
        assert_eq!(dir.to_string(), "/a");

        dir.path.push("b".to_string());
        assert_eq!(dir.to_string(), "/a/b");
    }

    #[test]
    fn directory_intoiterator() {
        let dir = Directory::from(["a", "b", "c"]);
        let mut iter = dir.into_iter();

        assert_eq!(iter.next(), Some("/a/b/c".to_string()));
        assert_eq!(iter.next(), Some("/a/b".to_string()));
        assert_eq!(iter.next(), Some("/a".to_string()));
        assert_eq!(iter.next(), Some("/".to_string()));
        assert_eq!(iter.next(), None);
    }
}
