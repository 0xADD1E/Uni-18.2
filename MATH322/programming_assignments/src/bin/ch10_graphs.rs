#[macro_use]
extern crate log;
extern crate env_logger;
extern crate failure;
extern crate programming_assignments;
extern crate regex;

#[derive(Debug)]
struct Node {
    name: String,
    edges: Vec<String>, // A Vec<Node*> would def be the conventional way of doing this
                        //But this is easier on both the type/borrow checker, and kate-brain
}

fn main() -> Result<(), failure::Error> {
    env_logger::init();

    let file = programming_assignments::get_argv1()?;
    let file = std::path::Path::new(&file);
    let mut file = std::fs::File::open(file)?;
    let mut string = String::new();

    use std::io::Read;
    use std::iter::FromIterator;
    info!("Reading file {:?}", file);
    file.read_to_string(&mut string)?;
    let string = Vec::from_iter(string.split("\n"));
    let (l1, l2) = (string[0], string[1].replace(" ", ""));
    debug!("Got string {:?} with lines '{}', '{}'", string, l1, l2);

    use std::collections::{HashMap,HashSet};
    let mut nodes = HashMap::new();
    for symbol in l1.split(", ") {
        debug!("Adding a new node: {}", symbol);
        nodes.insert(symbol.to_string(), HashSet::new());
    }

    let re = regex::Regex::new(r"\((.+?),(.+?)\);")?;
    for (a, b) in re
        .captures_iter(&l2)
        .filter_map(|m| match (m.get(1), m.get(2)) {
            (Some(a), Some(b)) => {
                trace!("Successfully matched regex elems 2,3 as {:?},{:?}", a, b);
                Some((String::from(a.as_str()), String::from(b.as_str())))
            }
            _ => {
                error!("Found invalid match pattern on match {:?}", m);
                None
            }
        }) {
        debug!("Attempting to add edge {} to node {}", a, b);
        if let Some(node) = nodes.get_mut(&b.to_string()) {
            trace!("Successfully looked up node {}", b);
            node.insert(a.clone());
        }
        debug!("Attempting to add edge {} to node {}", b, a);
        if let Some(node) = nodes.get_mut(&a.to_string()){
            trace!("Successfully looked up node {}", a);
            node.insert(b);
        }
    }
    println!("{:?}", nodes);
    Ok(())
}
