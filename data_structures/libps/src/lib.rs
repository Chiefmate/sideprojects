pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod linked_list;

#[cfg(test)]
mod tests {
    use super::*;
	use crate::linked_list::LinkedList;
	// use std::convert::TryInto;

	#[test]
	fn insert_at_head_works() {
		let mut list = LinkedList::<i32>::new();
		let second_value = 2;
		list.insert_at_head(1);
		list.insert_at_head(second_value);
		println!("Linked List is {}", list);
		match list.get(0) {
			Some(val) => assert_eq!(*val, second_value),
			None => panic!("Expected to find {} at index 0", second_value),
		}
	}

	#[test]
    fn delete_head_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_head(second_value);
        list.insert_at_head(first_value);
        match list.delete_head() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {} at head", first_value),
        }

        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }
}