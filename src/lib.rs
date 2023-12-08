mod utils;
#[derive(Clone)]
struct BuddyNode {
    next: Option<Box<BuddyNode>>,
    size: usize,
    used: bool,
}

impl BuddyNode {
    fn new(size: usize) -> BuddyNode {
        BuddyNode {
            next: None,
            size,
            used: false,
        }
    }
    fn fit(&self, size: usize) -> bool {
        (!self.used) & (size <= self.size)
    }
}

struct BuddyTree {
    root: Box<BuddyNode>,
}

impl BuddyTree {
    fn new(size: usize) -> BuddyTree {
        BuddyTree{
            root: Box::new(BuddyNode::new(size))
        }
    }

    fn allocate(&mut self, size: usize) -> bool {
        let m_size = utils::memory_to_allocate(size);
        let mut node = &mut self.root;

        loop {
            if node.fit(m_size) {
                while node.size != m_size {
                    let mut new_node = BuddyNode::new(node.size / 2);
                    new_node.next = node.next.take();
                    node.next = Some(Box::new(new_node));
                    node.size = node.size / 2;
                }
                node.used = true;
                return true;
            }
            else {
                match node.next {
                    Some(ref mut next) => {
                        node = next;
                    },
                    None => {
                        return false;
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_calc1() {
        let result = utils::memory_to_allocate(10);
        assert_eq!(result, 16);
    }

    #[test]
    fn mem_calc2() {
        let result = utils::memory_to_allocate(8);
        assert_eq!(result, 8);
    }

    #[test]
    fn mem_calc4() {
        let result = utils::memory_to_allocate(7);
        assert_eq!(result, 8);
    }

    #[test]
    fn mem_calc5() {
        let result = utils::memory_to_allocate(9);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_allocate1() {
        let mut tree = BuddyTree::new(16);
        let result = tree.allocate(8);
        assert_eq!(result, true);
    }

    #[test]
    fn test_allocate2() {
        let mut tree = BuddyTree::new(16);
        let result = tree.allocate(8);
        assert_eq!(result, true);
        let result = tree.allocate(8);
        assert_eq!(result, true);
    }
    #[test]
    fn test_allocate3() {
        let mut tree = BuddyTree::new(128);
        for _ in 0..16 {
            let result = tree.allocate(8);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_allocate4() {
        let mut tree = BuddyTree::new(128);
        for _ in 0..16 {
            let result = tree.allocate(8);
            assert_eq!(result, true);
        }
        for _ in 0..4 {
            let result = tree.allocate(32);
            assert_eq!(result, false);
        }
    }
}
