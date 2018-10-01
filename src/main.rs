extern crate ress;
extern crate walkdir;

use ress::{Scanner, Punct};
use walkdir::{WalkDir};

use std::{
    env::args,
    path::PathBuf,
    fs::read_to_string,
    collections::HashMap,
};

fn main() {
    let mut args = args();
    let _ = args.next();
    let start = args.collect::<Vec<String>>().join(" ");
    println!("{}", start);
    let res = check_js("var i = 0;").unwrap();
}

fn check_files(start: String) -> Result<(), HashMap<PathBuf, Vec<usize>>> {
    let mut ret: HashMap<PathBuf, Vec<usize>> = HashMap::new();
    // loop over the directories in our path
    // set the min_depth to 1, so we will skip the
    // path passed in as `start` and filter any
    // invalid entries or entries that don't end with .js
    // loop over each of the js files
    // and try to read them to a string
    for entry in WalkDir::new(start).min_depth(1) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.ends_with(".js") {
                if let Ok(js) = read_to_string(path) {
                    if let Err(indexes) = check_js(&js) {
                        ret.insert(path.to_path_buf(), indexes);
                    }
                }
            }

        }
    }
    if ret.len() > 0 {
        Err(ret)
    } else {
        Ok(())
    }
}

fn check_js(js: &str) -> Result<(), Vec<usize>> {
    // Create a scanner with the text
    let s = Scanner::new(js);
    // filter out any tokens that are not semi-colons
    // then collect them all into a `Vec` of the start index
    // for the semi-colon
    let semis: Vec<usize> = s.filter_map(|item| {
        // If this token matches the `Punct::SemiColon`
        if item.token.matches_punct(Punct::SemiColon) {
            // we want to return the first position of this token
            // since semi-colons are only 1 character wide we would
            // only need this part of the `Span`
            Some(item.span.start)
        } else {
            None
        }
    }).collect();
    // If we have anything in the result of the `filter_map`
    // we will return an error
    if  semis.len() > 0 {
        Err(semis)
    } else {
        Ok(())
    }
}