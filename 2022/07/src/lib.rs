use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i64,
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    files: Vec<Rc<RefCell<File>>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    size: i64,
    sized: bool,
}

enum Cmd {
    CD(String),
    LS,
}

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = vec![Rc::new(RefCell::new(Dir {
        name: "/".to_string(),
        files: vec![],
        dirs: vec![],
        size: 0,
        sized: false,
    }))];

    for line in input.lines().skip(2) {
        let mut cmd = Cmd::LS;
        if line.starts_with("$") {
            let (_, cmd_with_args) = line.split_once(" ").unwrap();
            if cmd_with_args.starts_with("cd") {
                cmd = Cmd::CD(cmd_with_args.replace("cmd", "").trim().to_string())
            }

            if let Cmd::CD(name) = cmd {
                let d = dir_stack.last().unwrap();

                let d = d.borrow_mut();

                let to_add = d.dirs.iter().find(|i| {
                    let i = i.borrow_mut();
                    i.name == name
                });

                if let Some(item) = to_add {
                    dir_stack.push(item.clone());
                }
            }
        } else {
            let (type_or_size, name) = line.split_once(" ").unwrap();

            let stack_top = dir_stack.last().unwrap().get_mut();
            if type_or_size == "dir" {
                stack_top.dirs.push(Rc::new(RefCell::new(Dir {
                    name: name.to_string(),
                    files: vec![],
                    dirs: vec![],
                    size: 0,
                    sized: false,
                })))
            } else {
                stack_top.files.push(Rc::new(RefCell::new(File {
                    name: name.to_string(),
                    size: type_or_size.parse().unwrap(),
                })))
            }
        }
    }
    0
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 95437);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 0);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 0);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
