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
        let mut vals = line.split(' ');

        match vals.next().unwrap() {
            "$" => match vals.next().unwrap() {
                "cd" => match vals.next().unwrap() {
                    "/" => cwd.clear(),
                    ".." => match cwd.rfind('/') {
                        Some(split) => {
                            // erase from the / onwards
                            cwd.truncate(split);
                        }
                        None => cwd.clear(),
                    },
                    into_dir => {
                        cwd.push('/');
                        cwd.push_str(into_dir);
                    }
                },
                "ls" => {
                    // about to get listing for cwd, clear out stored size
                    dir_sizes.insert(cwd.to_owned(), 0 as usize);
                }
                unknown => panic!("unknown line $ {}", unknown),
            },
            "dir" => (), // ignored.
            filesize => {
                // add to cwd's size.
                let filesize: usize = filesize.parse().unwrap();
                // filename ignored, don't need to track it since we clear dir size on seeing `ls`

                let cur_size = dir_sizes.get_mut(&cwd).unwrap();
                *cur_size += filesize;
            }
        }
    }

    // work out size of files from each child directory. BTree cause i want ordered printing
    let mut dir_total_sizes = BTreeMap::<String, usize>::new();
    for (mut dir, size) in dir_sizes {
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

    println!("dirs with size <= 100000");
    let mut total_sizes = 0;
    for (dir, size) in dir_total_sizes {
        if size <= 100000 && size != 0 {
            println!("{0: <7} {1}", size, &dir);
            total_sizes += size;
        }
    }

    println!("total sum: {}", total_sizes);
}
