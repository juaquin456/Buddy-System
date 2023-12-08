use buddy_system::BuddyTree;

fn main() {
    let mut tree = BuddyTree::new(16);
    for i in 0..16 {
        let result = tree.allocate(1, i);
        tree.print(0);
        assert_eq!(result, true);
    }
    for i in 0..4 {
        let result = tree.allocate(32, i+16);
        assert_eq!(result, false);
    }
}