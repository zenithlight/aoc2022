use std::env;
use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{digit1, multispace0, multispace1, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded, tuple};

#[derive(Debug)]
enum Prompt<'a> {
    ChangeDirectory {
        target: &'a str,
    },
    ListDirectory {
        entries: Vec<Entry<'a>>,
    },
}

#[derive(Debug)]
enum Entry<'a> {
    Directory {
        name: &'a str,
    },
    File {
        name: &'a str,
        size: usize,
    },
}

#[derive(Debug)]
struct FileTreeNode<'a> {
    size: usize,
    children: Option<HashMap<&'a str, FileTreeNode<'a>>>
}

fn parse_prompts(input: &str) -> IResult<&str, Vec<Prompt>> {
    many0(parse_prompt)(input)
}

fn parse_prompt<'a>(input: &'a str) -> IResult<&'a str, Prompt> {
    delimited(
        tag("$ "),
        alt((
            parse_change_directory,
            parse_list_directory
        )),
        multispace0
    )(input)
}

fn parse_change_directory(input: &str) -> IResult<&str, Prompt> {
    map(
        preceded(
            tag("cd "),
            take_while1(|c: char| !c.is_whitespace())
        ),
        |dir: &str| {
            Prompt::ChangeDirectory {
                target: dir
            }
        }
    )(input)
}

fn parse_list_directory(input: &str) -> IResult<&str, Prompt> {
    map(
        tuple((
            tag("ls"),
            take_while(|c: char| c.is_whitespace()),
            separated_list0(multispace1, parse_entry)
        )),
        |(_, _, entries)| Prompt::ListDirectory { entries }
    )(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        parse_directory,
        parse_file
    ))(input)
}

fn parse_directory(input: &str) -> IResult<&str, Entry> {
   map(
       preceded(
           tag("dir "),
           take_while1(|c: char| !c.is_whitespace())
       ),
       |dir| Entry::Directory { name: dir }
   )(input)
}

fn parse_file<'a>(input: &'a str) -> IResult<&str, Entry> {
    map(
        tuple((
            digit1::<&'a str, _>,
            space1,
            take_while1(|c: char| !c.is_whitespace())
        )),
        |(size, _, name)| Entry::File{ size: size.parse::<usize>().unwrap(), name }
    )(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input));

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input));
}

fn build_tree(input: &String) -> FileTreeNode {
    let mut root = FileTreeNode {
        size: 0,
        children: Some(HashMap::new())
    };

    let mut path = vec![];
    let prompts = parse_prompts(&input).unwrap().1;

    for prompt in prompts {
        match prompt {
            Prompt::ChangeDirectory { target } => {
                match target {
                    "/" => path.clear(),
                    ".." => { path.pop(); },
                    target => path.push(target)
                }
            },
            Prompt::ListDirectory { entries } => {
                let current: &mut FileTreeNode = find(&mut root, &path);
                for entry in entries {
                    match entry {
                        Entry::Directory { name } => {
                            current.children.as_mut().unwrap().insert(name, FileTreeNode {
                                size: 0,
                                children: Some(HashMap::new())
                            });
                        },
                        Entry::File { name, size } => {
                            current.children.as_mut().unwrap().insert(name, FileTreeNode {
                                size,
                                children: None
                            });
                        }
                    }
                }
            }
        }
    }

    root
}

fn find<'a, 'b, 'c, 'd>(root: &'a mut FileTreeNode<'b>, path: &'c Vec<&'d str>) -> &'a mut FileTreeNode<'b> {
    let mut current = root;

    for &directory in path {
        current = current.children.as_mut().unwrap().get_mut(directory).unwrap();
    }

    current
}

fn calculate_total_sizes(current: &mut FileTreeNode) {
    if let Some(children) = &mut current.children {
        current.size = children.values_mut().map(|entry| {
            calculate_total_sizes(entry);
            entry.size
        }).sum();
    }
}

fn calculate_a(current: &FileTreeNode) -> usize {
    match &current.children {
        None => 0, // we're only counting directories in this total
        Some(children) => {
            let self_contribution = if current.size <= 100_000 {
                current.size
            } else {
                0
            };

            let children_contribution: usize = children
                .values()
                .map(|entry| calculate_a(entry))
                .sum();

            self_contribution + children_contribution
        }
    }
}

fn walk_b<'a>(current: &'a FileTreeNode<'a>, space_needed_to_free: usize) -> Option<&'a FileTreeNode<'a>> {
    match &current.children {
        None => None,
        Some(children) => {
            if let Some(smallest_qualifying_child) = children
                .values()
                .filter(|entry| entry.children.is_some())
                .filter(|entry| entry.size >= space_needed_to_free)
                .min_by_key(|entry| entry.size) {
                walk_b(smallest_qualifying_child, space_needed_to_free)
            } else {
                if current.size >= space_needed_to_free {
                    Some(current) // base case
                } else {
                    None
                }
            }
        }
    }
}

fn a(input: String) -> usize {
    let mut root = build_tree(&input);
    calculate_total_sizes(&mut root);
    calculate_a(&root)
}

fn b(input: String) -> usize {
    static TOTAL_SPACE: usize = 70_000_000;
    static SPACE_NEEDED: usize = 30_000_000;

    let mut root = build_tree(&input);
    calculate_total_sizes(&mut root);

    let remaining_space = TOTAL_SPACE - root.size;
    let space_needed_to_free = SPACE_NEEDED - remaining_space;

    walk_b(&root, space_needed_to_free).unwrap().size
}

#[test]
fn test_example_a() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(a(input), 95437);
}

#[test]
fn test_example_b() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(b(input), 24933642);
}