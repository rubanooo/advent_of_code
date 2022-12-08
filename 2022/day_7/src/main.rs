const DIR_SIZE_THRESHOLD: u32 = 100_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const NEEDED_DISK_SPACE: u32 = 30_000_000;

fn main() {
    let input = include_str!("./input.txt");
    let file_system = parse_input(input);

    println!("Problem one: {}", problem_one(&file_system));
    println!("Problem two: {}", problem_two(&file_system));
}

fn problem_one(file_system: &FileSystem) -> u32 {
    file_system
        .directory_sizes()
        .filter(|size| size <= &DIR_SIZE_THRESHOLD)
        .sum()
}

fn problem_two(file_system: &FileSystem) -> u32 {
    let min_size_to_delete = NEEDED_DISK_SPACE - (TOTAL_DISK_SPACE - file_system.used_space());

    file_system
        .directory_sizes()
        .filter(|size| size >= &min_size_to_delete)
        .min()
        .expect("No directory found that can be deleted")
}

struct FileSystem {
    files: Vec<Entry>,
}

impl FileSystem {
    fn new() -> Self {
        Self { files: vec![] }
    }

    fn directory_sizes(&self) -> impl Iterator<Item = u32> + '_ {
        self.files.iter().filter_map(|entry| match entry {
            Entry::File(_) => None,
            Entry::Directory(directory) => Some(directory.size(self)),
        })
    }

    fn used_space(&self) -> u32 {
        self.files
            .iter()
            .filter_map(|entry| match entry {
                Entry::File(file) => Some(file.size),
                Entry::Directory(_) => None,
            })
            .sum()
    }
}

enum Entry {
    File(File),
    Directory(Directory),
}

struct File {
    path: String,
    size: u32,
}

struct Directory {
    path: String,
    name: String,
}

impl Directory {
    fn full_path(&self) -> String {
        if self.path.is_empty() {
            self.name.clone()
        } else {
            format!("{}/{}", self.path, self.name)
        }
    }

    fn children<'a>(&'a self, fs: &'a FileSystem) -> impl Iterator<Item = &Entry> {
        let path = self.full_path();

        fs.files.iter().filter(move |file| match file {
            Entry::File(file) => file.path == path,
            Entry::Directory(dir) => dir.path == path,
        })
    }

    fn size(&self, fs: &FileSystem) -> u32 {
        self.children(fs)
            .map(|entry| match entry {
                Entry::File(file) => file.size,
                Entry::Directory(dir) => dir.size(fs),
            })
            .sum()
    }
}

fn parse_input(input: &str) -> FileSystem {
    let mut current_dir: Vec<&str> = Vec::new();
    let mut file_system = FileSystem::new();

    for line in input.lines() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();

        match args[..] {
            ["$", "cd", "/"] => {
                current_dir = Vec::new();
            }
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", dir] => current_dir.push(dir),
            ["$", "ls"] => (),
            ["dir", name] => {
                file_system.files.push(Entry::Directory(Directory {
                    path: current_dir.join("/"),
                    name: name.to_string(),
                }));
            }
            [size, _name] => {
                file_system.files.push(Entry::File(File {
                    path: current_dir.join("/"),
                    size: size.parse().unwrap(),
                }));
            }
            _ => panic!("Can't parse {}", line),
        }
    }

    file_system
}
