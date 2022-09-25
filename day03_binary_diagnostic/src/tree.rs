use std::str::Chars;

pub struct Tree {
    pub size: i32,
    pub bit0: Option<Box<Tree>>,
    pub bit1: Option<Box<Tree>>
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            size: 0,
            bit0: None,
            bit1: None
        }
    }

    pub fn insert(&mut self, mut chars: Chars) {
        self.size += 1;
        if let Some(bit) = chars.next() {
            match bit {
                '0' => {
                    if let Some(tree0) = &mut self.bit0 {
                        tree0.insert(chars);
                    }
                    else {
                        let mut new_bit0 = Box::new(Tree::new());
                        new_bit0.insert(chars);
                        self.bit0 = Some(new_bit0);
                    }
                }
                '1' => {
                    if let Some(tree1) = &mut self.bit1 {
                        tree1.insert(chars);
                    }
                    else {
                        let mut new_bit1 = Box::new(Tree::new());
                        new_bit1.insert(chars);
                        self.bit1 = Some(new_bit1);
                    }
                }
                _ => panic!("Inserted value contains character other than 0 or 1: {}", bit)
            }
        }
    }
}