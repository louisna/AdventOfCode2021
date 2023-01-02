use std::{collections::{HashMap, LinkedList, HashSet}, cell::RefCell};

#[derive(Debug)]
struct Directory {
    name: String,
    subdir: HashSet<String>,
    files: HashSet<String>,
    total_values: u64,
}

fn main() {
    let content = include_str!("../../inputs/07.txt");
    let lines: Vec<&str> = content.split("\n").collect();

    let root = Box::new(RefCell::new(Directory {
        name: "/".to_string(),
        subdir: HashSet::new(),
        files: HashSet::new(),
        total_values: 0,
    }));

    let mut directories = HashMap::new();
    directories.insert("/".to_string(), root);

    let mut current = directories.get_mut("/").unwrap();
    let mut parents = LinkedList::new();

    for line in lines.iter().skip(1) {
        if line.starts_with("$") {
            let mut split = line.split(" ").skip(1);
            match split.next().unwrap() {
                "ls" => {
                    
                },
                "cd" => {
                    let name = split.next().unwrap();
                    current = match name {
                        ".." => directories.get_mut(&parents.pop_front().unwrap()).unwrap(),
                        "/" => {
                            parents = LinkedList::new();
                            directories.get_mut("/").unwrap()
                        }
                        other => {
                            parents.push_front(current.get_mut().name.clone());
                            let o = parents.front().unwrap_or(&"".to_string()).to_string() + other;
                            directories.get_mut(&o).unwrap()
                        },
                    };
                }
                _ => panic!("Error"),
            }
        } else {
            let mut split = line.split(" ");
            match split.next().unwrap() {
                "dir" => {
                    let dir_name = split.next().unwrap();
                    let dir_name = current.as_ref().borrow().name.clone() + dir_name;
                    if current.as_ref().borrow().subdir.contains(&dir_name) {
                        continue;
                    }

                    // Create new dir.
                    let new_dir = Box::new(RefCell::new(Directory {
                        name: dir_name.to_string(),
                        subdir: HashSet::new(),
                        files: HashSet::new(),
                        total_values: 0,
                    }));
                    
                    // Link from parent.
                    current.get_mut().subdir.insert(dir_name.to_string());
                    let current_name = current.get_mut().name.clone();
                    // parents.push_front(current.get_mut().name.clone());
                    
                    directories.insert(dir_name.to_string(), new_dir);

                    // Change the current.
                    current = directories.get_mut(&current_name).unwrap();
                },
                nb => {
                    let val: u64 = nb.parse().unwrap();
                    let name = split.next().unwrap();
                    if current.as_ref().borrow().files.contains(name) {
                        continue;
                    }
                    current.as_mut().borrow_mut().files.insert(name.to_string());

                    parents.push_front(current.get_mut().name.clone());

                    // Iterate over all the parents to update the value.
                    for par in parents.iter().rev() {
                        current = directories.get_mut(par).unwrap();
                        current.get_mut().total_values += val;
                    }

                    let name = parents.pop_front().unwrap();
                    current = directories.get_mut(&name).unwrap();
                }
            }
        }
    }

    let total = directories.values().filter(|dir| dir.as_ref().borrow().total_values <= 100_000).fold(0, |s, v| s + v.as_ref().borrow().total_values);
    println!("Total: {}", total);

    let remaining_size = 70000000 - directories.get("/").unwrap().as_ref().borrow().total_values;
    let need = 30000000 - remaining_size;

    let total = directories.values().filter(|dir| dir.as_ref().borrow().total_values > need).map(|d| d.as_ref().borrow().total_values).min().unwrap();
    println!("Total 2: {}", total);
}
