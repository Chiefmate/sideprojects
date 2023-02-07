use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct TreeNode<T> {
    value: Option<T>,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode<T> {
    pub fn new() -> TreeNode {
        TreeNode {
            value: None,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    pub fn print(&self) -> String {
        if let Some(value) = self.value {
            value.to_string()
        } else {
            String::from("[")
            +
            &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
            + "]";
        }
    }
}

fn init_tree(s: String) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    let chars = s.chars().collect::Vec<Vec<char>>();
    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
    {
        if *c == '[' || c.is_numeric() {
            let child = Rc::new(RefCell::new(TreeNode::new()));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
                if c.is_numeric() {
                    mut_child.value = c.to_digit(10);
                }
            }
            current = child;
        } else if *c == ',' || *c == ']' {
            let current_clone = Rc::clone(&current);
            current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        } else {
            panic!("Unknown character: {}", c);
        }
    }
    root.print()
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tree_1() {
        let tree = init_tree(String::from("[1,2]"));
        assert_eq!(tree.borrow().children[0].borrow().value.unwrap(), 1);
    }
}
