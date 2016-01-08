use std::fmt::{Debug, Formatter, Error};

use super::Tree;
use super::iter::Edge;

impl<T: Debug> Tree<T> {
    fn debug(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "Tree {{"));

        for edge in self.root().traverse() {
            try! {
                match edge {
                    Edge::Open(node) if node.has_children() => {
                        write!(f, " {:?} => {{", node.value())
                    },
                    Edge::Open(node) if node.next_sibling().is_some() => {
                        write!(f, " {:?},", node.value())
                    },
                    Edge::Open(node) => {
                        write!(f, " {:?}", node.value())
                    },

                    Edge::Close(node) if node.has_children() => {
                        if node.next_sibling().is_some() {
                            write!(f, " }},")
                        } else {
                            write!(f, " }}")
                        }
                    },

                    _ => Ok(()),
                }
            }
        }

        write!(f, " }}")
    }

    fn debug_alternate(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut i = String::from("    ");

        try!(write!(f, "Tree {{"));

        for edge in self.root().traverse() {
            try! {
                match edge {
                    Edge::Open(node) if node.has_children() => {
                        let result = write!(f, "\n{}{:#?} => {{", i, node.value());
                        i.push_str("    ");
                        result
                    },
                    Edge::Open(node) => {
                        write!(f, "\n{}{:#?},", i, node.value())
                    },

                    Edge::Close(node) if node.has_children() => {
                        let len = i.len() - 4;
                        i.truncate(len);
                        if node.parent().is_some() {
                            write!(f, "\n{}}},", i)
                        } else {
                            write!(f, "\n{}}}", i)
                        }
                    },

                    _ => Ok(()),
                }
            }
        }

        write!(f, "\n}}")
    }
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if f.alternate() {
            self.debug_alternate(f)
        } else {
            self.debug(f)
        }
    }
}
