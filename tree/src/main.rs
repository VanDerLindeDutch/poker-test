
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: u64,
}

fn main() {
    let height = 2;
    let head = generate_tree_dfs(0, height);
    let mut buf = vec![0;height as usize + 1];
    calculate_dfs(Some(head), &mut buf, 0);
    buf.iter().for_each(|x| {println!("{}", x)})
}


fn generate_tree_dfs(cur_i: u64, height: u32) -> Box<Node> {
    if height == 0 {
        return Box::new(Node {
            left: None,
            right: None,
            val: cur_i,
        });
    }
    let size = 1u64 << (height);
    Box::new(Node {
        left: Some(generate_tree_dfs(cur_i + 1, height - 1)),
        right: Some(generate_tree_dfs(cur_i + size, height - 1)),
        val: cur_i,
    })
}

fn calculate_dfs(node: Option<Box<Node>>, buf: &mut [u64], height: u32) {
    match node {
        None => {}
        Some(node) => {
            buf[height as usize] += node.val;
            calculate_dfs(node.left, buf, height + 1);
            calculate_dfs(node.right, buf, height + 1);
        }
    }
}
fn print(node: &Option<Box<Node>>, indent: String, is_left: bool) {
    if node.is_none() {
        return;
    }
    let node = node.as_ref().unwrap();
    println!("{}|___{}\n", indent, node.val);
    let newStr = if is_left {
        indent + "|    "
    } else {
        indent + "   "
    };
    print(&node.left, newStr.clone(), true);
    print(&node.right, newStr, false)
}