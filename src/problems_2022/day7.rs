use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day7/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 7:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}
#[derive(Debug, Clone, PartialEq)]
enum Item {
    Dir((Option<usize>, String)),
    File((usize, String)),
}

impl Item {
    fn get_name(&self) -> &String {
        match self {
            Item::Dir((_, name)) => name,
            Item::File((_, name)) => name,
        }
    }

    fn get_size(&self) -> usize {
        match self {
            Item::Dir((size, _)) => size.expect("Size to be computed"),
            Item::File((size, _)) => *size,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Item::Dir(_) => true,
            Item::File(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
enum Command {
    cd(String),
    ls(Vec<Item>),
}

type Input = Vec<Command>;

fn parse(input: &str) -> Input {
    input
        .split('$')
        .filter(|command| !command.is_empty())
        .map(|command| match command.trim() {
            cd if cd.starts_with("cd") => Command::cd(cd[2..].trim().to_string()),
            ls if ls.starts_with("ls") => Command::ls(
                ls[2..]
                    .split('\n')
                    .filter_map(|line| {
                        if line.trim().is_empty() {
                            None
                        } else {
                            Some(line.trim())
                        }
                    })
                    .map(|line| match line {
                        dir if dir.starts_with("dir") => {
                            Item::Dir((None, dir[3..].trim().to_string()))
                        }
                        file if file.chars().next().map_or(false, |c| c.is_numeric()) => {
                            if let Some((size, name)) = file.split_whitespace().collect_tuple() {
                                Item::File((
                                    size.parse::<usize>()
                                        .expect("Expected a number as file size"),
                                    name.to_string(),
                                ))
                            } else {
                                panic!("Cannot split {} into size and name", file);
                            }
                        }
                        unmatched => panic!("{} does not match any command", unmatched),
                    })
                    .collect_vec(),
            ),
            unmatched => panic!("{} does not match any command", unmatched),
        })
        .collect_vec()
}

#[derive(Debug)]
struct Node {
    pub item: Item,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(item: Item, parent: Option<Weak<RefCell<Node>>>) -> Node {
        Node {
            item,
            children: vec![],
            parent,
        }
    }
    fn get_child(&self, name: &String) -> Option<&Rc<RefCell<Node>>> {
        self.children
            .iter()
            .find(|child| child.borrow().item.get_name() == name)
    }

    fn compute_size(&mut self) -> usize {
        match &mut self.item {
            Item::File((size, _)) => *size,
            Item::Dir((size, _)) => {
                let computed_size = self
                    .children
                    .iter()
                    .map(|child| child.borrow_mut().compute_size())
                    .sum();
                *size = Some(computed_size);
                computed_size
            }
        }
    }
}

fn build_tree(input: Input) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new(
        Item::Dir((None, "/".to_string())),
        None,
    )));
    let mut current_dir = root.clone();
    for command in input {
        match command {
            Command::cd(dest) => match dest.as_str() {
                "/" => current_dir = root.clone(),
                ".." => {
                    let parent = current_dir
                        .borrow()
                        .parent
                        .as_ref()
                        .expect("Parent to exist")
                        .upgrade()
                        .expect("Parent to not be destroyed");
                    current_dir = parent;
                }
                _ => {
                    let child = current_dir
                        .borrow()
                        .get_child(&dest)
                        .expect("Dir to exist")
                        .clone();
                    current_dir = child;
                }
            },
            Command::ls(content) => {
                for item in content.into_iter() {
                    match item.clone() {
                        Item::Dir(_) => {
                            let new_node = Rc::new(RefCell::new(Node::new(
                                item,
                                Some(Rc::downgrade(&current_dir)),
                            )));
                            current_dir.borrow_mut().children.push(new_node.clone());
                        }
                        Item::File((_, name)) => {
                            if current_dir.borrow().get_child(&name).is_none() {
                                let new_node = Rc::new(RefCell::new(Node::new(
                                    item,
                                    Some(Rc::downgrade(&current_dir)),
                                )));
                                current_dir.borrow_mut().children.push(new_node);
                            }
                        }
                    }
                }
            }
        }
    }
    root.borrow_mut().compute_size();
    root
}

fn solve_part1(input: Input) -> usize {
    let tree = build_tree(input);
    let mut queue: Vec<Rc<RefCell<Node>>> = vec![tree];
    let mut sum = 0;
    while let Some(dir) = queue.pop() {
        let size = dir.borrow_mut().item.get_size();

        if size <= 100000 {
            sum += size;
        }
        for child in dir.borrow().children.iter() {
            if child.borrow().item.is_dir() {
                queue.push(child.clone());
            }
        }
    }
    sum
}

fn solve_part2(input: Input) -> usize {
    let tree = build_tree(input);
    let mut queue: Vec<Rc<RefCell<Node>>> = vec![tree];
    let mut dirs = vec![];
    while let Some(dir) = queue.pop() {
        dirs.push(dir.clone());
        for child in dir.borrow().children.iter() {
            if child.borrow().item.is_dir() {
                queue.push(child.clone());
            }
        }
    }

    let total_size = 70000000;
    let needed_size = 30000000;
    let current_size: usize = dirs
        .iter()
        .find(|dir| dir.borrow().item.get_name() == "/")
        .expect("Root to exist")
        .borrow()
        .item
        .get_size();
    let free = total_size - current_size;
    let missing = needed_size - free;
    dirs.into_iter()
        .filter_map(|dir| match dir.borrow().item.get_size() {
            x if x < missing => None,
            x if x >= missing => Some(x),
            _ => unreachable!(),
        })
        .min()
        .expect("One dir to be larger than the missing space")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part1(parse(include_str!("day7/example_1.txt"))),
            95437
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 1084134);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
