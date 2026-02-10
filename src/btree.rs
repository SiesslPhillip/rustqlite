use std::io::{Error, ErrorKind};

#[derive(Clone, Debug)]
pub struct Location {
    pub(crate) byte_range_start: i64,
    pub(crate) byte_range_stop: i64,
}

#[derive(Debug)]
pub struct ValueStruct {
    pub(crate) value: i64,
    pub(crate) location: Vec<Location>,
}

#[derive(Debug)]
pub struct BTree {
    values: Vec<ValueStruct>,
    children: Vec<Box<BTree>>,
    leaf: bool,
}

impl BTree {
    pub fn new() -> Box<BTree> {
        Box::new(BTree {
            values: vec![],
            children: vec![],
            leaf: true,
        })
    }

    pub fn insert_into_tree(root: &mut Box<BTree>, input: ValueStruct) -> Result<bool, Error> {
        match root.insert_recursive(input)? {
            None => Ok(true),
            Some((promoted, right_sibling)) => {
                let old_root = std::mem::replace(
                    root,
                    Box::new(BTree {
                        values: vec![],
                        children: vec![],
                        leaf: true,
                    }),
                );

                let new_root = BTree {
                    values: vec![promoted],
                    children: vec![old_root, right_sibling],
                    leaf: false,
                };
                *root = Box::new(new_root);
                Ok(true)
            }
        }
    }

    /// Recursive insert.
    ///
    /// Returns:
    /// - Ok(None) if insertion completed without splitting this node
    /// - Ok(Some((promoted_key, right_sibling))) if THIS node split and must be
    ///   inserted into the parent.
    fn insert_recursive(
        &mut self,
        mut input: ValueStruct,
    ) -> Result<Option<(ValueStruct, Box<BTree>)>, Error> {
        if let Some(idx) = self.values.iter().position(|v| v.value == input.value) {
            self.values[idx].location.append(&mut input.location);
            return Ok(None);
        }

        // insert key here, then split if overfull
        if self.leaf {
            let pos = self
                .values
                .iter()
                .position(|v| v.value > input.value)
                .unwrap_or(self.values.len());
            self.values.insert(pos, input);

            if self.values.len() <= 2 {
                return Ok(None);
            }

            let mut keys = std::mem::take(&mut self.values); // len == 3

            let k0 = keys.remove(0);
            let k1 = keys.remove(0); // promoted
            let k2 = keys.remove(0);

            self.values = vec![k0];

            let right = BTree {
                values: vec![k2],
                children: vec![],
                leaf: true,
            };

            return Ok(Some((k1, Box::new(right))));
        }

        if self.children.len() != self.values.len() + 1 {
            return Err(Error::new(
                ErrorKind::Other,
                "BTree invariant violated: children.len() != values.len() + 1",
            ));
        }

        let child_index = self.child_index_for(input.value);
        let split = self.children[child_index].insert_recursive(input)?;

        if let Some((promoted, right_sibling)) = split {
            self.values.insert(child_index, promoted);
            self.children.insert(child_index + 1, right_sibling);

            if self.values.len() <= 2 {
                return Ok(None);
            }

            // Split internal node with 3 keys and 4 children.
            // keys: [k0, k1, k2], children: [c0, c1, c2, c3]
            let mut keys = std::mem::take(&mut self.values);
            let mut kids = std::mem::take(&mut self.children);

            let k0 = keys.remove(0);
            let k1 = keys.remove(0); // promoted
            let k2 = keys.remove(0);

            let c0 = kids.remove(0);
            let c1 = kids.remove(0);
            let c2 = kids.remove(0);
            let c3 = kids.remove(0);

            self.values = vec![k0];
            self.children = vec![c0, c1];
            self.leaf = false;

            let right = BTree {
                values: vec![k2],
                children: vec![c2, c3],
                leaf: false,
            };

            return Ok(Some((k1, Box::new(right))));
        }

        Ok(None)
    }

    /// Decide which child to descend into for a key that is NOT contained in this node.
    /// Works for 0, 1, or 2 keys.
    fn child_index_for(&self, value: i64) -> usize {
        match self.values.len() {
            0 => 0,
            1 => {
                if value < self.values[0].value {
                    0
                } else {
                    1
                }
            }
            2 => {
                if value < self.values[0].value {
                    0
                } else if value < self.values[1].value {
                    1
                } else {
                    2
                }
            }
            _ => unreachable!("2-3 tree nodes should not have >2 keys outside split handling"),
        }
    }

    pub fn search_tree(&self, value: i64) -> Result<Vec<Location>, Error> {
        if let Some(v) = self.values.iter().find(|v| v.value == value) {
            return Ok(v.location.clone());
        }

        if self.leaf {
            return Ok(vec![]);
        }

        // Internal => descend
        if self.children.len() != self.values.len() + 1 {
            return Err(Error::new(
                ErrorKind::Other,
                "BTree invariant violated: children.len() != values.len() + 1",
            ));
        }

        let idx = self.child_index_for(value);
        self.children[idx].search_tree(value)
    }
}

// let mut tree = BTree::new();
//
// BTree::insert_into_tree(
// &mut tree,
// ValueStruct {
// value: 10,
// location: vec![Location {
//     byte_range_start: 12,
//     byte_range_stop: 15,
// }],
// },
// )?;
//
// BTree::insert_into_tree(
// &mut tree,
// ValueStruct {
// value: 5,
// location: vec![Location {
//     byte_range_start: 1,
//     byte_range_stop: 2,
// }],
// },
// )?;
//
// BTree::insert_into_tree(
// &mut tree,
// ValueStruct {
// value: 20,
// location: vec![Location {
//     byte_range_start: 100,
//     byte_range_stop: 200,
// }],
// },
// )?;
//
// BTree::insert_into_tree(
// &mut tree,
// ValueStruct {
// value: 10,
// location: vec![Location {
//     byte_range_start: 30,
//     byte_range_stop: 40,
// }],
// },
// )?;
//
// let hits = tree.search_tree(10)?;
// println!("hits for 10: {:?}", hits);
//
// Ok(())
