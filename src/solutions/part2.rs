enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum TreeNode<'a> {
    Leaf(i32),
    Expr {
        op: Op,
        left: &'a TreeNode<'a>,
        right: &'a TreeNode<'a>,
    },
}

fn evaluate(tree: &TreeNode) -> i32 {
    match *tree {
        TreeNode::Leaf(ref num) => *num,
        TreeNode::Expr {
            op: Op::Add,
            ref left,
            ref right,
        } => evaluate(left) + evaluate(right),
        TreeNode::Expr {
            op: Op::Sub,
            ref left,
            ref right,
        } => evaluate(left) - evaluate(right),
        TreeNode::Expr {
            op: Op::Mul,
            ref left,
            ref right,
        } => evaluate(left) * evaluate(right),
        TreeNode::Expr {
            op: Op::Div,
            ref left,
            ref right,
        } => evaluate(left) / evaluate(right),
    }
}

pub fn main() {
    let leaf_120 = TreeNode::Leaf(120);
    let leaf_2 = TreeNode::Leaf(2);
    let leaf_3 = TreeNode::Leaf(3);
    let leaf_5 = TreeNode::Leaf(5);
    let leaf_8 = TreeNode::Leaf(8);

    let t1 = TreeNode::Expr {
        op: Op::Div,
        left: &leaf_120,
        right: &leaf_2,
    };
    let t2 = TreeNode::Expr {
        op: Op::Mul,
        left: &leaf_3,
        right: &leaf_5,
    };
    let t3 = TreeNode::Expr {
        op: Op::Sub,
        left: &t2,
        right: &leaf_8,
    };

    let tree = TreeNode::Expr {
        op: Op::Add,
        left: &t1,
        right: &t3,
    };
    println!("{}", evaluate(&tree));
}
