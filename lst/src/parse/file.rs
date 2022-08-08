use nom::combinator::all_consuming;

use crate::lst::*;
use crate::nom::*;

use super::block::block;

pub fn file(t: &str) -> NResult {
    let (_, n) = all_consuming(block)(t)?;
    Ok(SynNode::new_root(Node::new(File.into(), vec![n])))
}

#[cfg(test)]
mod tests {
    use std::io::{stdin, Read};

    use super::*;

    #[test]
    fn test_playground() {
        let mut buf = String::new();
        stdin()
            .lock()
            .read_to_string(&mut buf)
            .expect("cannot open stdin");
        match file(&buf) {
            Ok(v) => println!("\n=======\n{:#?}", v),
            Err(e) => println!("\n\nErr:\n{}", e),
        }
    }
}
