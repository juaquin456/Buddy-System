#[cfg(test)]
mod tests {
    use crate::buddy::BuddyTree;
    use crate::utils;

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
        let result = tree.allocate(8, "A");
        assert_eq!(result, true);
    }

    #[test]
    fn test_allocate2() {
        let mut tree = BuddyTree::new(16);
        let result = tree.allocate(8, "A");
        assert_eq!(result, true);
        let result = tree.allocate(8, "B");
        assert_eq!(result, true);
    }

    #[test]
    fn test_allocate3() {
        let mut tree = BuddyTree::new(128);
        for i in 0..16 {
            let result = tree.allocate(8, i);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_allocate4() {
        let mut tree = BuddyTree::new(128);
        for i in 0..16 {
            let result = tree.allocate(8, i);
            assert_eq!(result, true);
        }
        for i in 0..4 {
            let result = tree.allocate(32, i + 16);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn test_deallocate1() {
        let mut tree = BuddyTree::new(128);
        for i in 0..16 {
            let result = tree.allocate(8, i);
            assert_eq!(result, true);
        }
        for i in 0..4 {
            let result = tree.allocate(32, i + 16);
            assert_eq!(result, false);
        }
        for i in 0..16 {
            let result = tree.deallocate(i);
            assert_eq!(result, true);
        }
    }
    
    #[test]
    fn test_deallocate2() {
        let mut tree = BuddyTree::new(128);
        for i in 0..16 {
            let result = tree.deallocate(i);
            assert_eq!(result, false);
        }
    }
    
    #[test]
    fn test_deallocate3() {
        let mut tree = BuddyTree::new(128);
        for i in 0..16 {
            let result = tree.allocate(8, i);
            assert_eq!(result, true);
        }
        for i in 0..16 {
            let result = tree.deallocate(i);
            assert_eq!(result, true);
        }
        for i in 0..16 {
            let result = tree.allocate(8, i);
            assert_eq!(result, true);
        }
    }
    
    #[test]
    fn test_global_use() {
        let mut tree = BuddyTree::new(128);
        assert_eq!(tree.allocate(8, "A"), true);
        assert_eq!(tree.allocate(16, "B"), true);
        assert_eq!(tree.allocate(32, "C"), true);
        assert_eq!(tree.allocate(64, "D"), true);
        assert_eq!(tree.deallocate("B"), true);
        assert_eq!(tree.allocate(8, "E"), true);
        assert_eq!(tree.deallocate("0"), false);
    }
}