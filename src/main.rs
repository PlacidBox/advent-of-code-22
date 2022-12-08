use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut cwd = "".to_owned();
    let mut dir_sizes = HashMap::new();

    // load size of files directly under each directory.
    for line in lines {
        let line = line.unwrap();
        let vals: Vec<&str> = line.split(' ').collect();

        match vals.as_slice() {
            ["$", "cd", "/"] => cwd.clear(),
            ["$", "cd", ".."] => {
                match cwd.rfind('/') {
                    Some(split) => {
                        // erase from the / onwards
                        cwd.truncate(split);
                    }
                    None => cwd.clear(),
                }
            }
            ["$", "cd", into_dir] => {
                cwd.push('/');
                cwd.push_str(into_dir);
            }
            ["$", "ls"] => {
                // about to get listing for cwd, clear out stored size
                dir_sizes.insert(cwd.to_owned(), 0 as usize);
            }
            ["dir", _dir_name] => (), // ignored.
            [filesize, _file_name] => {
                // add to cwd's size.
                let filesize: usize = filesize.parse().unwrap();
                // filename ignored, don't need to track it since we clear dir size on seeing `ls`

                let cur_size = dir_sizes.get_mut(&cwd).unwrap();
                *cur_size += filesize;
            }
            _ => panic!("unknown line $ {}", &line),
        }
    }

    // work out size of files from each child directory. BTree cause i want ordered printing
    let mut dir_total_sizes = BTreeMap::<String, usize>::new();
    let mut used_space = 0;
    for (mut dir, size) in dir_sizes {
        used_space += size;

        while !dir.is_empty() {
            match dir_total_sizes.get_mut(&dir) {
                Some(total_size) => *total_size += size,
                None => {
                    dir_total_sizes.insert(dir.to_owned(), size);
                }
            }

            // knock off trailing dir, equiv to 'cd ..'
            match dir.rfind('/') {
                Some(split) => dir.truncate(split),
                None => dir.clear(),
            }
        }
    }

    const FS_SIZE: usize = 70000000;
    const NEED_SPACE: usize = 30000000;
    let used_space = used_space;
    let space_avail = FS_SIZE - used_space;
    let need_to_free = NEED_SPACE - space_avail;
    println!(
        "fs takes up {} of {}, need to free {} to hit {} avail bytes",
        used_space, FS_SIZE, need_to_free, NEED_SPACE
    );

    for (dir, size) in dir_total_sizes {
        if size > need_to_free {
            println!("could delete {0:<7} {1}", size, dir);
        }
    }
}
