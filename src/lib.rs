#[derive(PartialEq, Clone)]
enum State {
    Used,
    NoUsed,
    PartialUsed
}

#[derive(Clone)]
struct BuddyNode {
    next: Option<Box<BuddyNode>>,
    prev: Option<Box<BuddyNode>>,
    size: usize,
    state: State,
}

impl BuddyNode {
    fn new(size: usize) -> BuddyNode {
        BuddyNode {
            prev: None,
            next: None,
            size,
            state: State::NoUsed,
        }
    }
    fn fit(&self, size: usize) -> bool {
        (self.state != State::Used) & (size < self.size)
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
        todo!("Implementar")
    }

}

fn memory_to_allocate(size: usize) -> usize {
    if size == 0 {
        return 0;
    }

    let mut n = size;
    let mut longest_bit = 0;

    loop {
        n >>= 1;
        longest_bit += 1;

        if n == 0 {
            break;
        }
    }
    let potency_of_2: usize = 1 << (longest_bit - 1);
    if potency_of_2 >= size {
        potency_of_2
    }
    else {
        potency_of_2 << 1
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_calc1() {
        let result = memory_to_allocate(10);
        assert_eq!(result, 16);
    }

    #[test]
    fn mem_calc2() {
        let result = memory_to_allocate(8);
        assert_eq!(result, 8);
    }

    #[test]
    fn mem_calc4() {
        let result = memory_to_allocate(7);
        assert_eq!(result, 8);
    }

    #[test]
    fn mem_calc5() {
        let result = memory_to_allocate(9);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_buddy_tree() {
        let mut tree = BuddyTree::new(16);
        let result = tree.allocate(8);
        assert_eq!(result, true);
    }
}
