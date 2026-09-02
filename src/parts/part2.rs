// Start by uncommenting this,
// which will be used to represent operators in the expression.
/*
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
*/

// TODO:
// Make a enum `TreeNode` that can be a leaf (Leaf) or an expression (Expr).
// If it's a Leaf, it will contain one integer (i32).
// If it's an Expr, it will contain an operation (Op) and two sub-expressions (TreeNode).
// Start by uncommenting the block below.

/*
enum TreeNode {
    // Complete this!
}

fn evaluate(tree: &TreeNode) -> i32 {
    // Complete this as well!
}
*/

pub fn main() {
    // Uncomment below when you are ready.
    /*
    let leaf_120 = TreeNode::Leaf(120);
    let leaf_2 = TreeNode::Leaf(2);
    let leaf_3 = TreeNode::Leaf(3);
    let leaf_5 = TreeNode::Leaf(5);
    let leaf_8 = TreeNode::Leaf(8);

    let t1 = TreeNode::Expr { op: Op::Div, left: &leaf_120, right: &leaf_2 };
    let t2 = TreeNode::Expr { op: Op::Mul, left: &leaf_3, right: &leaf_5 };
    let t3 = TreeNode::Expr { op: Op::Sub, left: &t2, right: &leaf_8 };

    let tree = TreeNode::Expr { op: Op::Add, left: &t1, right: &t3 };
    println!("{}", evaluate(&tree));
    */
}
