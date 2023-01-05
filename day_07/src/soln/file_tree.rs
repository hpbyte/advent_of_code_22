use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub size: u32,
    pub is_dir: bool,
    pub children: Vec<File>,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            is_dir: true,
            children: vec![],
        }
    }

    // locate a file node
    fn find(&mut self, name: &str) -> &mut Self {
        let mut stack = vec![];
        stack.push(self);

        loop {
            let node = stack.pop().unwrap();

            if node.name == name {
                println!("{}", node.name);
                break node;
            }

            for child in &mut node.children {
                stack.push(child);
            }
        }
    }

    // recursively get all the children
    pub fn flat_children(&self) -> HashSet<Self> {
        let mut set = HashSet::new();

        for child in &self.children {
            set.insert(child.clone());
            set.extend(child.flat_children());
        }

        set
    }

    // change the size of the parent folder
    fn aggregate_size(&mut self) {
        for child in &mut self.children {
            child.aggregate_size();
            // change the parent size
            self.size += child.size;
        }
    }

    // append child file
    fn append(&mut self, node: File) {
        self.children.push(node);
    }

    // pre-order traversal
    fn traverse_pre_order(&self) -> Vec<String> {
        let mut vec = vec![];

        fn traverse(node: &File, vec: &mut Vec<String>) {
            let mut s = String::from(node.name.clone());
            s.push_str(" ");
            s.push_str(node.size.clone().to_string().as_str());
            vec.push(s);

            for child in &node.children {
                traverse(child, vec);
            }
        }

        traverse(self, &mut vec);

        vec
    }

    // print the tree
    pub fn print(&self) {
        let tree = self
            .traverse_pre_order()
            .iter()
            .fold(String::from(""), |mut accu, curr| {
                accu.push_str(curr);
                accu.push_str(", ");
                accu
            });

        println!("{}", tree);
    }
}

/*
* create a file tree based on the commands
* returns the root
*/
pub fn create_file_tree(raw: String) -> File {
    // create the root folder
    let mut tree = File::new("/");

    let mut path: Vec<&str> = vec!["/"];

    for line in raw.lines().skip(1) {
        let cmd_line: Vec<&str> = line.split_whitespace().collect();

        // command line
        if line.starts_with("$") {
            match cmd_line[1] {
                "cd" => {
                    // change directory
                    debug_assert!(cmd_line.len() == 3, "There should be three parts!");

                    if cmd_line[2] == ".." {
                        path.pop();
                    } else {
                        path.push(cmd_line[2]);
                    }

                    let dir_name = path.last().unwrap();
                    let parent = tree.find(dir_name);
                    if parent
                        .children
                        .iter()
                        .any(|x| x.name == dir_name.to_string())
                    {
                        continue;
                    }

                    let new_dir = File::new(dir_name);
                    parent.children.push(new_dir);
                }
                "ls" => {
                    continue;
                }
                _ => unreachable!(),
            };

            continue;
        }

        debug_assert!(cmd_line.len() == 2, "There should be two parts!");

        // it's a directory
        if line.starts_with("dir") {
            let parent = tree.find(path.last().unwrap());

            if parent
                .children
                .iter()
                .find(|child| child.name == cmd_line[1])
                != None
            {
                continue;
            }

            let new_dir = File::new(cmd_line[1]);

            // append a child
            parent.append(new_dir);
            continue;
        }

        // it's a file
        let mut new_file = File::new(cmd_line[1]);
        new_file.size = cmd_line[0].parse().unwrap_or_else(|_| 0);
        new_file.is_dir = false;

        tree.find(path.last().unwrap()).append(new_file);
    }

    tree.aggregate_size();
    tree
}
