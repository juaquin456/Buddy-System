use buddy_system_visualization::BuddyTree;

fn main() {
    let mut tree = BuddyTree::new(16);
    
    tree.allocate(2, 0);
    tree.print(0);
    tree.allocate(3, 1);
    tree.print(0);
    tree.deallocate(1);
    tree.print(0);
    tree.deallocate(0);
    tree.print(0);
}