use std::rc::Rc;
use std::cell::RefCell;
use std::process::exit;
use std::mem::take;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red = 1,
    Black = 0,
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug, PartialEq)]
struct Node {
    value: i32,
    color: Color,
    parent: Option<NodeRef>, 
    left: Option<NodeRef>,   
    right: Option<NodeRef>,  
}

impl Node {
    fn new(value: i32, color: Color) -> NodeRef {
        Rc::new(RefCell::new(Node {
            value,
            color,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

#[derive(Debug)]
struct SavedNodeInfo {
    saved_node: Option<NodeRef>, 
    is_left_or_right_child: i32,
    deleted_node_color: Color,
}

fn preorder(root: Option<&NodeRef>) {
    if let Some(root_ref) = root {
        let node = root_ref.borrow();
        print!("{}-{}", node.value, node.color as i32);
        match &node.parent {
            None => print!("-p:N "),
            Some(parent_ref) => {
                let parent = parent_ref.borrow();
                print!("-p:{} ", parent.value);
            },
        }
        preorder(node.left.as_ref());
        preorder(node.right.as_ref());
    }
}

fn check_red_black_tree(root: Option<&NodeRef>) -> i32 {
    match root {
        None => 0, 
        Some(root_ref) => {
            let root_node = root_ref.borrow();
            
            if root_node.parent.is_none() && root_node.color == Color::Red {
                return -1;
            }
            
            // Kiểm tra tính đỏ đen của cây con trái
            let leftroute = check_red_black_tree(root_node.left.as_ref());
            if leftroute == -1{
                return -1;
            }

            // Kiểm tra tính đỏ đen của cây con phải
            let rightroute = check_red_black_tree(root_node.right.as_ref());
            if rightroute == -1{
                return -1;
            }

            // Kiểm tra hai nút đỏ liên tiếp
            if root_node.color == Color::Red {
                if let Some(left) = &root_node.left {
                    if left.borrow().color == Color::Red {
                        return -1;
                    }
                }
                if let Some(right) = &root_node.right {
                    if right.borrow().color == Color::Red {
                        return -1;
                    }
                }
            }
            
            if leftroute != rightroute {
                return -1;
            }
            
            leftroute + (if root_node.color == Color::Red { 0 } else { 1 })
        }
    }
}

fn insert_norm_bst(
    root: Option<&NodeRef>, 
    parent: Option<&NodeRef>, 
    item: i32, 
    newnode: &mut SavedNodeInfo
) -> Option<NodeRef> {
    match root {
        None => {
            let new_node = Node::new(item, Color::Red);
            
            if let Some(parent_ref) = parent {
                let mut new_node_mut = new_node.borrow_mut();
                new_node_mut.parent = Some(Rc::clone(parent_ref));
            }
            
            newnode.saved_node = Some(Rc::clone(&new_node));
            
            Some(new_node)
        },
        Some(root_ref) => {
            let mut root_node = root_ref.borrow_mut();
            
            if item < root_node.value {
                let left_result = insert_norm_bst(
                    root_node.left.as_ref(), 
                    root,
                    item, 
                    newnode
                );
                root_node.left = left_result;
            } else if item > root_node.value {
                let right_result = insert_norm_bst(
                    root_node.right.as_ref(), 
                    root,
                    item, 
                    newnode
                );
                root_node.right = right_result;
            }
            Some(Rc::clone(root_ref))
        }
    }
}

fn red_black_tree_insertion_cover(
    root: &mut Option<NodeRef>,
    newnode: &mut Option<NodeRef>,
    debug: i32,
) -> Option<NodeRef> {
    if newnode.is_none() || root.is_none() {
        return std::mem::take(root);
    }
    if Rc::ptr_eq(newnode.as_ref().unwrap(), root.as_ref().unwrap()) {
        newnode.as_ref().unwrap().borrow_mut().color = Color::Black;
        return root.clone();
    }

    if newnode.as_ref().unwrap().borrow().parent.is_none() {
        eprintln!("Parent of new node must exist!");
        std::process::exit(1);
    }

    let mut parent = newnode.as_ref().unwrap().borrow().parent.clone().unwrap();
    if parent.borrow().color == Color::Red {
        let grandparent_ref = parent.borrow().parent.clone();
        if grandparent_ref.is_none() {
            eprintln!("Parent of red node must exist!");
            exit(1);
        }
        let grandparent = grandparent_ref.unwrap();

        if debug > 0 {
            println!(
                "node={}, cha={}, ong={}",
                newnode.as_ref().unwrap().borrow().value,
                parent.borrow().value,
                grandparent.borrow().value
            );
        }

        let mut uncle = None;
        let grandparent_left = grandparent.borrow().left.clone();
        let grandparent_right = grandparent.borrow().right.clone();

        if let Some(ref left) = grandparent_left {
            if Rc::ptr_eq(&parent, left) {
                uncle = grandparent_right.clone();
            }
            else {
                uncle = grandparent_left.clone();
            }
        }

        if uncle.is_none() {
            uncle = Some(Node::new(i32::MAX, Color::Black));
            if debug > 0 {
                println!("make fantom");
            }
        }

        if uncle.as_ref().unwrap().borrow().color == Color::Red {
            if debug > 0 {
                println!(
                    "TH chu do node={}, cha={}, ong={}",
                    newnode.as_ref().unwrap().borrow().value,
                    parent.borrow().value,
                    grandparent.borrow().value
                );
            }
            grandparent.borrow_mut().color = Color::Red;
            uncle.as_ref().unwrap().borrow_mut().color = Color::Black;
            parent.borrow_mut().color = Color::Black;
            return red_black_tree_insertion_cover(root, &mut Some(grandparent.clone()), debug);
        } else {
            let is_parent_left = grandparent_left
                .as_ref()
                .map_or(false, |left| Rc::ptr_eq(left, &parent));

            if is_parent_left {
                if !parent.borrow().right.is_none(){
                    if Rc::ptr_eq(&newnode.as_ref().unwrap(), &parent.borrow().right.as_ref().unwrap()) {
                        if debug > 0 {
                            println!(
                                "TH LR node={}, cha={}, ong={}",
                                newnode.as_ref().unwrap().borrow().value,
                                parent.borrow().value,
                                grandparent.borrow().value
                            );
                        }
                        parent.borrow_mut().right = newnode.as_ref().unwrap().borrow().left.clone();
                        if !newnode.as_ref().unwrap().borrow().left.is_none(){
                            newnode.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().parent = Some(parent.clone());
                        }
                        newnode.as_ref().unwrap().borrow_mut().left = Some(parent.clone());
                        grandparent.borrow_mut().left = newnode.clone();
                        let temp = newnode.clone();
                        *newnode = Some(parent.clone());
                        parent = temp.as_ref().unwrap().clone();
                        newnode.as_ref().unwrap().borrow_mut().parent = Some(parent.clone());
                        parent.borrow_mut().parent = Some(grandparent.clone());
                    }
                }
                if debug > 0 {
                    println!(
                        "TH LL node={}, cha={}, ong={}",
                        newnode.as_ref().unwrap().borrow().value,
                        parent.borrow().value,
                        grandparent.borrow().value
                    );
                }
                parent.borrow_mut().color = Color::Black;
                grandparent.borrow_mut().color = Color::Red;
                let parent_of_grandparent = grandparent.borrow().parent.clone();
                if !parent_of_grandparent.is_none() {
                    if Rc::ptr_eq(&grandparent, &parent_of_grandparent.as_ref().unwrap().borrow().left.as_ref().unwrap()){
                        parent_of_grandparent.as_ref().unwrap().borrow_mut().left = Some(parent.clone());
                    }
                    else {
                        parent_of_grandparent.as_ref().unwrap().borrow_mut().right = Some(parent.clone());
                    }
                }
                else {
                    *root = Some(parent.clone());
                }
                grandparent.borrow_mut().left = parent.borrow().right.clone();
                if !parent.borrow().right.is_none(){
                    parent.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(grandparent.clone());
                }
                parent.borrow_mut().parent = parent_of_grandparent.clone();
                parent.borrow_mut().right = Some(grandparent.clone());
                grandparent.borrow_mut().parent = Some(parent);
            } else {
                if !parent.borrow().left.is_none(){
                    if Rc::ptr_eq(&newnode.as_ref().unwrap(), &parent.borrow().left.as_ref().unwrap()) {
                        if debug > 0 {
                            println!(
                                "TH RL node={}, cha={}, ong={}",
                                newnode.as_ref().unwrap().borrow().value,
                                parent.borrow().value,
                                grandparent.borrow().value
                            );
                        }
                        parent.borrow_mut().left = newnode.as_ref().unwrap().borrow().right.clone();
                        if !newnode.as_ref().unwrap().borrow().right.is_none(){
                            newnode.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().parent = Some(parent.clone());
                        }
                        newnode.as_ref().unwrap().borrow_mut().right = Some(parent.clone());
                        grandparent.borrow_mut().right = newnode.clone();
                        let temp = newnode.clone();
                        *newnode = Some(parent.clone());
                        parent = temp.as_ref().unwrap().clone();
                        newnode.as_ref().unwrap().borrow_mut().parent = Some(parent.clone());
                        parent.borrow_mut().parent = Some(grandparent.clone());
                    }
                }
                if debug > 0 {
                    println!(
                        "TH RR node={}, cha={}, ong={}",
                        newnode.as_ref().unwrap().borrow().value,
                        parent.borrow().value,
                        grandparent.borrow().value
                    );
                }
                parent.borrow_mut().color = Color::Black;
                grandparent.borrow_mut().color = Color::Red;
                let parent_of_grandparent = grandparent.borrow().parent.clone();
                if !parent_of_grandparent.is_none() {
                    if Rc::ptr_eq(&grandparent, &parent_of_grandparent.as_ref().unwrap().borrow().left.as_ref().unwrap()){
                        parent_of_grandparent.as_ref().unwrap().borrow_mut().left = Some(parent.clone());
                    }
                    else {
                        parent_of_grandparent.as_ref().unwrap().borrow_mut().right = Some(parent.clone());
                    }
                }
                else {
                    *root = Some(parent.clone());
                }
                grandparent.borrow_mut().right = parent.borrow().left.clone();
                if !parent.borrow().left.is_none(){
                    parent.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(grandparent.clone());
                }
                parent.borrow_mut().parent = parent_of_grandparent.clone();
                parent.borrow_mut().left = Some(grandparent.clone());
                grandparent.borrow_mut().parent = Some(parent);
            }
        }

        if uncle.as_ref().unwrap().borrow().value == i32::MAX {
            drop(uncle); 
        }
    }

    return std::mem::take(root);
}

fn insert(mut root: Option<NodeRef>, x: i32, debug: i32) -> Option<NodeRef> {
    let mut newnode = SavedNodeInfo { saved_node: None, is_left_or_right_child: 0, deleted_node_color: Color::Black};
    root = insert_norm_bst(root.as_ref(), None, x, &mut newnode);
    if debug > 0 {
        if let Some(saved_node) = &newnode.saved_node {
            let saved_node_borrow = saved_node.borrow();
            println!("\nnutvuachen: {}", saved_node_borrow.value);

            if let Some(parent) = &saved_node_borrow.parent {
                println!("nutcha: {}", parent.borrow().value);

                if let Some(grandparent) = &parent.borrow().parent {
                    println!("nutong: {}", grandparent.borrow().value);
                }
            }
        } else {
            println!("\nkhong the chen {}", x);
        }
    }

    if let Some(saved_node) = newnode.saved_node {
        root = red_black_tree_insertion_cover(&mut root, &mut Some(saved_node), debug);
    } else {
        eprintln!("\nLoi, khong the chen {}", x);
        exit(1);
    }
    return root;
}

fn find_first_right_none( 
    root: Option<&NodeRef>
) -> Option<NodeRef> {
    if root.as_ref().unwrap().borrow().right.is_none(){
        return root.cloned();
    }
    return find_first_right_none(root.as_ref().unwrap().borrow().right.as_ref());
}

fn find_first_left_none( 
    root: Option<&NodeRef>
) -> Option<NodeRef> {
    if root.as_ref().unwrap().borrow().left.is_none(){
        return root.cloned();
    }
    return find_first_right_none(root.as_ref().unwrap().borrow().left.as_ref());
}

fn delete_norm_bst(
    root: Option<&NodeRef>,       
    parent: Option<&NodeRef>,    
    item: i32,                   
    newnode: &mut SavedNodeInfo, 
    successor: i32,              
) -> Option<NodeRef> {
    match root {
        None => return None,
        Some(root_ref) => {
            let value = {
                let root_node = root_ref.borrow();
                root_node.value
            };
            
            if item < value{
                let root_node = root_ref.borrow();
                let left_result = delete_norm_bst(
                    root_node.left.as_ref(),
                    root,
                    item,
                    newnode,
                    successor
                );
                drop(root_node);
                root_ref.borrow_mut().left = left_result;
            }
            else if item > value{
                let root_node = root_ref.borrow();
                let right_result = delete_norm_bst(
                    root_node.right.as_ref(),
                    root,
                    item,
                    newnode,
                    successor
                );
                drop(root_node);
                root_ref.borrow_mut().right = right_result;
            }
            else {
                let is_left_child: bool = {
                    root.as_ref().unwrap().borrow().left.is_none()
                };
                let is_right_child: bool = {
                    root.as_ref().unwrap().borrow().right.is_none()
                };
                if is_left_child && is_right_child{ // nut la
                    let mut lr = 0;
                    let parent_clone = {
                        &root.as_ref().unwrap().borrow().parent
                    };
                    if let Some(parent_clone_ref) = parent_clone {
                        if !root.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.is_none() && Rc::ptr_eq(&root.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap(), root.as_ref().unwrap()) {
                                lr = 1;
                        }
                        else if let Some(right) = parent_clone_ref.borrow().right.as_ref() {
                            if Rc::ptr_eq(&right, root.as_ref().unwrap()) {
                                lr = 2;
                            }
                        }
                        else {
                            print!(" Loi nghiem trong\n");
                            exit(1);
                        }
                    }
                    newnode.saved_node = Some(Node::new(i32::MAX, Color::Black));
                    newnode.saved_node.as_ref().unwrap().borrow_mut().parent = parent.clone().cloned();
                    newnode.is_left_or_right_child = lr;
                    newnode.deleted_node_color = root.as_ref().unwrap().borrow().color;
                    return None;
                }
                else if is_right_child { // ton tai nut con ben trai
                    newnode.deleted_node_color = {
                        root.as_ref().unwrap().borrow().color
                    };
                    let tmp = {
                        &root.as_ref().unwrap().borrow().left
                    };
                    tmp.as_ref().unwrap().borrow_mut().parent = parent.clone().cloned();
                    newnode.saved_node = tmp.clone();
                    return tmp.clone();
                }
                else if is_left_child {
                    newnode.deleted_node_color = { // ton tai nut con ben phai
                        root.as_ref().unwrap().borrow().color
                    };
                    let tmp = {
                        &root.as_ref().unwrap().borrow().right
                    };
                    tmp.as_ref().unwrap().borrow_mut().parent = parent.clone().cloned();
                    newnode.saved_node = tmp.clone();
                    return tmp.clone();
                }
                else { // co hai nut la
                    if successor == 0{
                        let tmp = {
                            &root.as_ref().unwrap().borrow().left
                        };
                        if tmp.as_ref().unwrap().borrow().right.is_none(){
                            let value = tmp.as_ref().unwrap().borrow().value;
                            let color = root.as_ref().unwrap().borrow().color;
                            let root_right_child = {
                                root.as_ref().unwrap().borrow().right.clone()
                            };
                            let new_root = Node::new(value, color);
                            new_root.borrow_mut().right = root_right_child.clone();
                            new_root.borrow_mut().left = root.as_ref().unwrap().borrow().left.clone();
                            let new_root_left = delete_norm_bst(
                                tmp.as_ref(), 
                                Some(&new_root), 
                                tmp.as_ref().unwrap().borrow().value, 
                                newnode, 
                                successor
                            );
                            new_root.borrow_mut().left = new_root_left.clone();
                            new_root.borrow_mut().parent = root.as_ref().unwrap().borrow().parent.clone();
                            root_right_child.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            // print!("Day {}", newnode.saved_node.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().value);
                            return Some(new_root);
                        }
                        else {
                            let tmp_clone;
                            tmp_clone = find_first_right_none(tmp.as_ref());
                            let value = tmp_clone.as_ref().unwrap().borrow().value;
                            let color = root.as_ref().unwrap().borrow().color;
                            let new_root = Node::new(value, color);
                            let root_right_child = {
                                root.as_ref().unwrap().borrow().right.clone()
                            };
                            new_root.borrow_mut().right = root.as_ref().unwrap().borrow().right.clone();
                            drop(tmp_clone);
                            let new_root_left = delete_norm_bst(root.as_ref().unwrap().borrow().left.as_ref() ,root, value, newnode, successor);
                            new_root_left.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            new_root.borrow_mut().left = new_root_left.clone();
                            new_root.borrow_mut().parent = root.as_ref().unwrap().borrow().parent.clone();
                            root_right_child.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            new_root_left.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            return Some(new_root.clone());
                        }
                    }
                    else {
                        let tmp = {
                            &root.as_ref().unwrap().borrow().right
                        };
                        if tmp.as_ref().unwrap().borrow().left.is_none(){
                            let value = tmp.as_ref().unwrap().borrow().value;
                            let color = root.as_ref().unwrap().borrow().color;
                            let root_right_child = {
                                root.as_ref().unwrap().borrow().left.clone()
                            };
                            let new_root = Node::new(value, color);
                            new_root.borrow_mut().left = root.as_ref().unwrap().borrow().left.clone();
                            let new_root_left = delete_norm_bst(
                                tmp.as_ref(), 
                                root, 
                                tmp.as_ref().unwrap().borrow().value, 
                                newnode, 
                                successor
                            );
                            new_root.borrow_mut().right = new_root_left.clone();
                            new_root.borrow_mut().parent = root.as_ref().unwrap().borrow().parent.clone();
                            root_right_child.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            return Some(new_root);
                        }
                        else {
                            let tmp_clone;
                            tmp_clone = find_first_left_none(tmp.as_ref());
                            let value = tmp_clone.as_ref().unwrap().borrow().value;
                            let color = root.as_ref().unwrap().borrow().color;
                            let new_root = Node::new(value, color);
                            let root_right_child = {
                                root.as_ref().unwrap().borrow().left.clone()
                            };
                            new_root.borrow_mut().left = root.as_ref().unwrap().borrow().left.clone();
                            drop(tmp_clone);
                            let new_root_left = delete_norm_bst(root.as_ref().unwrap().borrow().right.as_ref() ,root, value, newnode, successor);
                            new_root_left.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            new_root.borrow_mut().right = new_root_left.clone();
                            new_root.borrow_mut().parent = root.as_ref().unwrap().borrow().parent.clone();
                            root_right_child.as_ref().unwrap().borrow_mut().parent = Some(new_root.clone());
                            return Some(new_root.clone());
                        }
                    }
                }
                
            }
        }
    }
    root.cloned() 
}


fn rotate_on_parent(
    root: &mut Option<NodeRef>,
    newnode: Option<NodeRef>,
    s: &mut Option<NodeRef>,
    lor: i32,
    debug: i32,
) -> Option<NodeRef> {
    // let parent = newnode.as_ref().and_then(|node| node.borrow().parent.clone());
    let parent = newnode.as_ref().unwrap().borrow().parent.clone();
    let grandparent = parent.as_ref().unwrap().borrow().parent.clone();

    if let Some(grandparent_node) = &grandparent {

        let mut grandparent_borrow = grandparent_node.borrow_mut();

        if Rc::ptr_eq(&grandparent_borrow.left.as_ref().unwrap(), &parent.as_ref().unwrap()) {
            grandparent_borrow.left = s.clone();
        } 
        else if Rc::ptr_eq(&grandparent_borrow.right.as_ref().unwrap(), &parent.as_ref().unwrap()) {
            grandparent_borrow.right = s.clone();
        }
    } 
    else {
        if debug > 0 {
            println!("grandparent is NULL");
        }
        *root = s.clone();
    }

    if let (Some(parent_node), Some(s_node)) = (parent.as_ref(), s.as_ref()) {
        if lor != 0 {
            parent_node.borrow_mut().right = s_node.borrow().left.clone();
            if let Some(left_node) = s_node.borrow().left.as_ref() {
                left_node.borrow_mut().parent = parent.clone();
            }
            s_node.borrow_mut().left = parent.clone();
        } 
        else {
            parent_node.borrow_mut().left = s_node.borrow().right.clone();
            if let Some(right_node) = s_node.borrow().right.as_ref() {
                right_node.borrow_mut().parent = parent.clone();
            }
            s_node.borrow_mut().right = parent.clone();
        }
    }

    if let Some(s_node) = s.as_ref() {
        s_node.borrow_mut().parent = grandparent.clone();
    }
    if let Some(parent_node) = parent.as_ref() {
        parent_node.borrow_mut().parent = s.clone();
    }

    std::mem::take(root)
}

fn red_black_tree_deletion_cover(
    mut root: Option<NodeRef>,
    newnode: Option<NodeRef>,
    deleted_node_color: Color,
    lor: i32,
    debug: i32,
) -> Option<NodeRef> {
    if newnode.is_none() || root.is_none(){

        return root;
    }
    if deleted_node_color == Color::Red{
        return root;
    }
    // print!("{}",newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().color as i32);
    if Rc::ptr_eq(&newnode.as_ref().unwrap(), &root.as_ref().unwrap()) || newnode.as_ref().unwrap().borrow().color == Color::Red{
        newnode.as_ref().unwrap().borrow_mut().color = Color::Black;
        // print!("Lan 1");
        return root;
    }
    else {
        // Loi nam trong doan nay
        let mut s : Option<NodeRef> = None;
        let mut cs = 0;
        if newnode.as_ref().unwrap().borrow().value == i32::MAX{

            if lor == 1{
                s = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().right.clone();
                cs = 1;
            }
            else if lor == 2{
                s = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().left.clone();
                cs = 2;
            }
            else {
                cs -= 1;
            }
        }
        else {
            if Rc::ptr_eq(&newnode.as_ref().unwrap(), &newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()){
                s = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.clone();
                cs = 3;
            }
            else if Rc::ptr_eq(&newnode.as_ref().unwrap(), &newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap()){
                s = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.clone();
                cs = 4;
            }
            else {
                cs -= 2;
            }
        }
        if cs <= 0{
            print!("Co loi nghiem trong xay ra cs = {}\n", cs);
            exit(1);
        }
        if cs == 1 || cs == 3{
            if s.as_ref().unwrap().borrow().color == Color::Red{
                if debug > 0{
                    print!("NDK-TH1 ");  
                }
                s.as_ref().unwrap().borrow_mut().color = Color::Black;
                newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Red;
                root = rotate_on_parent(&mut root, newnode.as_ref().cloned(), &mut s, 1, debug);
                s = newnode.as_ref().unwrap().borrow().parent.as_ref().as_ref().unwrap().borrow().right.clone();
                if debug > 0{
                    print!("\nroot={}\n", root.as_ref().unwrap().borrow().value);
                    preorder(root.as_ref());
                    print!("\n");
                }
            }
            if (s.as_ref().unwrap().borrow().left.is_none() || s.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == Color::Black) && 
                    (s.as_ref().unwrap().borrow().right.is_none() || s.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == Color::Black){
                if debug > 0{
                    print!("NDK-TH2 ");
                }
                s.as_ref().unwrap().borrow_mut().color = Color::Red;
                // print!("{}", newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().value);
                root = red_black_tree_deletion_cover(root, newnode.as_ref().unwrap().borrow().parent.clone(), deleted_node_color, lor, debug);
            }
            else {
                if s.as_ref().unwrap().borrow().right.is_none() || s.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == Color::Black{
                    if debug > 0{
                        print!("NDK-TH3");
                    }
                    s.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().color = Color::Black;
                    s.as_ref().unwrap().borrow_mut().color = Color::Red;
                    let new_right = s.as_ref().unwrap().borrow().left.clone();
                    newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().right = new_right.clone();
                    new_right.as_ref().unwrap().borrow_mut().parent = newnode.as_ref().unwrap().borrow().parent.clone();
                    s.as_ref().unwrap().borrow_mut().left = new_right.as_ref().unwrap().borrow().right.clone();
                    if let Some(new_right_right_ref) = &new_right.as_ref().unwrap().borrow().right.as_ref(){
                        new_right_right_ref.borrow_mut().parent = s.clone();
                    };
                    new_right.as_ref().unwrap().borrow_mut().right = s.clone();
                    s.as_ref().unwrap().borrow_mut().parent = new_right.clone();
                    s = new_right.clone();
                    if debug > 0{
                        print!("\nroot={}\n", root.as_ref().unwrap().borrow().value);
                        preorder(root.as_ref());
                        print!("\n");
                    }
                }

                if debug > 0 {
                    print!("NDK-TH4\n");
                }
                s.as_ref().unwrap().borrow_mut().color = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().color;
                newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Black;
                s.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().color = Color::Black;

                root = rotate_on_parent(&mut root.clone(), newnode.clone(), &mut s.clone(), 1, debug);
                if debug > 0{
                    print!("\nroot={}\n",root.as_ref().unwrap().borrow().value);
                    preorder(root.as_ref());
                    print!("\n");
                }
            }
        }
        else{
            if s.as_ref().unwrap().borrow().color == Color::Red{
                if debug > 0{
                    print!("DK-NDK-TH1 ");
                }
                s.as_ref().unwrap().borrow_mut().color = Color::Black;
                newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Red;
                root = rotate_on_parent(&mut root.clone(), newnode.clone(), &mut s.clone(), 0, debug);
                s = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.clone();
                if debug > 0{
                    print!("\nroot={}\n",root.as_ref().unwrap().borrow().value);
                    preorder(root.as_ref());
                    print!("\n");
                }
            }
            if (s.as_ref().unwrap().borrow().left.is_none() || s.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == Color::Black) && 
                    (s.as_ref().unwrap().borrow().right.is_none() || s.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == Color::Black){
                if debug > 0{
                    print!("DK-NDK-TH2 ");
                }
                s.as_ref().unwrap().borrow_mut().color = Color::Red;
                root = red_black_tree_deletion_cover(root, newnode.as_ref().unwrap().borrow().parent.clone(), deleted_node_color, lor, debug);
            }
            else {
                if s.as_ref().unwrap().borrow().left.is_none() || s.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == Color::Black{
                    if debug > 0{
                        print!("DK-NDK-TH3");
                    }
                    s.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().color = Color::Black;
                    s.as_ref().unwrap().borrow_mut().color = Color::Red;
                    let new_left = s.as_ref().unwrap().borrow().right.clone();
                    newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().left = new_left.clone();
                    new_left.as_ref().unwrap().borrow_mut().parent = newnode.as_ref().unwrap().borrow().parent.clone();
                    s.as_ref().unwrap().borrow_mut().right = new_left.as_ref().unwrap().borrow().right.clone();
                    if let Some(new_left_left_ref) = &new_left.as_ref().unwrap().borrow().left.as_ref(){
                        new_left_left_ref.borrow_mut().parent = s.clone();
                    };
                    new_left.as_ref().unwrap().borrow_mut().left = s.clone();
                    s.as_ref().unwrap().borrow_mut().parent = new_left.clone();
                    s = new_left.clone();
                    if debug > 0{
                        print!("\nroot={}\n", root.as_ref().unwrap().borrow().value);
                        preorder(root.as_ref());
                        print!("\n");
                    }
                }

                if debug > 0{
                    print!("DK-NDK-TH4\n");
                }
                s.as_ref().unwrap().borrow_mut().color = newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().color;
                newnode.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Black;
                s.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().color = Color::Black;

                root = rotate_on_parent(&mut root.clone(), newnode.clone(), &mut s.clone(), 0, debug);
                if debug > 0{
                    print!("\nroot={}\n",root.as_ref().unwrap().borrow().value);
                    preorder(root.as_ref());
                    print!("\n");
                }
            }
        }
        if newnode.as_ref().unwrap().borrow().value == i32::MAX{
            drop(newnode);
        }
        return root;
    }
    // return root;
}

fn delete(
    root:&mut Option<NodeRef>,
    key: i32,
    debug: i32,
    successor: i32,
) -> Option<NodeRef> {
    let mut newnode = SavedNodeInfo {saved_node: None, is_left_or_right_child: 0, deleted_node_color: Color::Black};
    newnode.saved_node = None;
    if debug > 0{
        print!("\nnut se xoa: {} ", key);
    }
    *root = delete_norm_bst(root.clone().as_ref(), None, key,&mut newnode, successor);
    
    if debug >0 {
        if !newnode.saved_node.is_none(){
            print!("\nnutthaythe: {} ", newnode.saved_node.as_ref().unwrap().borrow().value);
            if !newnode.saved_node.as_ref().unwrap().borrow().parent.is_none(){
                print!("nutchacua nutthaythe:{} \n", newnode.saved_node.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().value);
            }
            else {
                print!("nutchacua nutthaythe:NULL \n");
            }
        }
        else {
            print!("khong the xoa nut \n");
        }
    }
    if debug > 0{
        print!("\nroot={}\n", root.as_ref().unwrap().borrow().value);
        preorder(root.as_ref());
        print!("\n");
    }

    if !newnode.saved_node.is_none(){
        // print!("{}", newnode.saved_node.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().value);
        *root = red_black_tree_deletion_cover(root.as_ref().cloned(), newnode.saved_node, newnode.deleted_node_color, newnode.is_left_or_right_child, debug);
    }
    else {
        print!("\nLoi, khong the xoa {}\n", key);
        exit(1);
    }
    return take(root);
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    println!("\n==========================Kiem thu thu cong:=================================\n");

    let mut root: Option<NodeRef> = None;

    let test_values = vec![50, 30, 55, 60, 62, 53, 35, 37, 31, 25, 10, 20, 23, 15, 13, 99, 1092];

    for &value in &test_values {
        root = insert(root, value, 0); 
        let is_valid = check_red_black_tree(root.as_ref()); 
        println!("\nKTdoden = {}", is_valid);
        preorder(root.as_ref()); 
        println!("\n");
    }
    for &value in &test_values {
        // print!("{}", value);
        root = delete(&mut root, value, 0, 0); 
        let is_valid = check_red_black_tree(root.as_ref()); 
        println!("\nKTdoden = {}", is_valid);
        preorder(root.as_ref()); 
        println!("\n");
    }
}
