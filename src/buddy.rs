use std::fmt::Display;
use std::usize;

use colored::Colorize;

use crate::utils;
use crate::utils::calculate_width;

#[derive(Clone)]
struct BuddyNode<T> {
    next: Option<Box<BuddyNode<T>>>,
    size: usize,
    used: bool,
    id: Option<T>,
}

impl<T> BuddyNode<T> where T: PartialEq + Copy {
    fn new(size: usize) -> BuddyNode<T> {
        BuddyNode {
            next: None,
            size,
            used: false,
            id: None,
        }
    }

    /// Returns true if the node is free and fits the given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size to check.
    fn fit(&self, size: usize) -> bool {
        (!self.used) & (size <= self.size)
    }

    /// Sets the id of the node and marks it as used.
    ///
    /// # Arguments
    ///
    /// * `id` - The id to set.
    fn set_id(&mut self, id: T) {
        self.id = Some(id);
        self.used = true;
    }

    /// Returns true if the node contains the given id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id to check.
    fn contains(&self, id: T) -> bool {
        match self.id {
            Some(ref i) => {
                *i == id
            }
            None => {
                false
            }
        }
    }

    /// Releases the node.
    fn release(&mut self) {
        self.id = None;
        self.used = false;
    }
}

pub struct BuddyTree<T> {
    root: Box<BuddyNode<T>>,
}

impl<T> BuddyTree<T> where T: PartialEq + Copy + Display {
    /// Creates a new BuddyTree with the specified size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the Buddy System.
    pub fn new(size: usize) -> BuddyTree<T> {
        BuddyTree {
            root: Box::new(BuddyNode::new(size))
        }
    }

    /// Allocates memory of the given size using the buddy memory allocation algorithm.
    ///
    /// Returns true if the allocation is successful, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the memory to allocate.
    /// * `id` - The id of the memory to allocate.
    pub fn allocate(&mut self, size: usize, id: T) -> bool {
        let m_size = utils::memory_to_allocate(size);
        let mut node = &mut self.root;

        loop {
            // First free node found which fits the size
            if node.fit(m_size) {
                while node.size != m_size {
                    // Split the node until the size is reached
                    let mut new_node = BuddyNode::new(node.size / 2);
                    new_node.next = node.next.take();
                    node.next = Some(Box::new(new_node));
                    node.size = node.size / 2;
                }

                // Mark the node as used
                node.set_id(id);
                return true;
            } else {
                match node.next {
                    Some(ref mut next) => {
                        node = next;
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    }

    /// Deallocates the memory with the given id.
    ///
    /// Returns true if the deallocation is successful, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the memory to deallocate.
    pub fn deallocate(&mut self, id: T) -> bool {
        let mut node = &mut self.root;
        loop {
            if node.contains(id) {
                node.release();
                return true;
            } else {
                match node.next {
                    Some(ref mut next) => {
                        node = next;
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    }

    /// Prints the BuddySystem
    ///
    /// # Arguments
    ///
    /// * `rescale` - The number of bits to rescale the size of the nodes. More rescale number means smaller nodes.
    pub fn print(&self, rescale: u32) {
        let mut node = &self.root;

        loop {
            print!("|{}|", format!("{: ^width$}", {
                match node.id {
                    Some(ref id) => {
                        format!("id  : {:4}", id).blue()
                    }
                    None => {
                        format!("size: {:4}", node.size).red()
                    }
                }
            }, width = 10 + 2 * calculate_width(node.size.ilog2() - rescale) as usize));


            match node.next {
                Some(ref next) => {
                    node = next;
                }
                None => {
                    println!();
                    break;
                }
            }
        }
    }
}