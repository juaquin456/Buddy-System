use buddy_system::BuddyTree;

fn main() {
    let mut tree = BuddyTree::new(16);
    for i in 0..4 {
        let result = tree.allocate(3, i);
        tree.print(2);
        assert_eq!(result, true);
    }
    for i in 0..4 {
        let result = tree.allocate(32, i+16);
        assert_eq!(result, false);
    }
}