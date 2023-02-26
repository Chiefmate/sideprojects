use std::ptr::NonNull;
// NonNull is a covariant of *mut T
// NonNull always be non-null, unlike *mut T
// Option<NonNull<T>> has the same size as NonNull<T>
use std::marker::PhantomData;
// PhantomData is a zero-sized type 
// used to mark things that "act like" they own a T
use std::fmt::{self, Display, Formatter};
// std::fmt for printing

struct Node<T> {
	val: T,
	next: Option<NonNull<Node<T>>>,
	prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
	fn new(t: T) -> Node<T> {
		Node {
			val: t,
			prev: None,
			next: None,
		}
	}
}

pub struct LinkedList<T> {
	length: u32,
	head: Option<NonNull<Node<T>>>,
	tail: Option<NonNull<Node<T>>>,
	// Act like we own boxed nodes since we construct and leak them
	marker: PhantomData<Box<Node<T>>>,
}

// Default trait for giving a type useful value
impl<T> Default for LinkedList<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> LinkedList<T> {
	// creates a linked list
	pub fn new() -> Self {
		Self {
			length: 0,
			head: None,
			tail: None,
			marker: PhantomData,
		}
	}

	// insert an object into the front of the linked list
	pub fn insert_at_head(&mut self, obj: T) {
		let mut node = Box::new(Node::new(obj));
		node.next = self.head;
		node.prev = None;

		// wrap a pointer with NonNull::new_unchecked()
		let node_ptr = Some(
			unsafe { 
				NonNull::new_unchecked(Box::into_raw(node))
			});
		match self.head {
			None => self.tail = node_ptr,
			Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
		}
		self.head = node_ptr;
		self.length += 1;
	}

	// deletes the head node
	// returns old_head val to check if the list is cleared
	pub fn delete_head(&mut self) -> Option<T> {
		// Safety: head_ptr points to a leaked boxed node managed by this list
		// We reassign pointers that pointed to the head node
		// Option::map maps an Option<T> to Option<T> wrapping old_head's value
		// Option::map takes self by value, consuming self.head
		self.head.map(|head_ptr| unsafe {
			let old_head = Box::from_raw(head_ptr.as_ptr());
			match old_head.next {
				Some(mut next_ptr) => next_ptr.as_mut().prev = None,
				None => self.tail = None,
			}
			self.head = old_head.next;
			self.length -= 1;
			old_head.val
		})
	}

	pub fn get(&mut self, index: i32) -> Option<&'static T> {
		Self::get_ith_node(self.head, index)
	}

	fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<&'static T> {
		match node {
			None => None,
			Some(next_ptr) => match index {
				0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
				_ => Self::get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
			}
		}
	}
}

// Drop trait for cleanup
impl<T> Drop for LinkedList<T> {
	fn drop(&mut self) {
		// Pop items until there are none left
		while self.delete_head().is_some() {}
	}
}

// Display trait
impl<T> Display for LinkedList<T> 
where
	T: Display,
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.head {
			Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
			None => Ok(()),
		}
	}
}

impl<T> Display for Node<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.next {
			Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
			None => write!(f, "{}", self.val),
		}
	}
}
