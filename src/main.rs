// fn main() {
//     struct Node {
//         value: i32,
//         next: Option<Box<Node>>,
//     }
//     struct List {
//         head: Option<Box<Node>>,
//     }

//     // impl Iterator for List {
//     //     type Item = i32;
//     //     fn next(&mut self) -> Option<Self::Item> {
//     //         match self.head {
//     //             Some(ref mut node) => {
//     //                 let node = node.next.take();
//     //                 // self.head = node;
//     //                 Some(node.unwrap().value)
//     //             }
//     //             None => None,
//     //         }
//     //     }
//     // }

//     let mut node1 = Node {
//         value: 1,
//         next: None,
//     };

//     let node2 = Node {
//         value: 2,
//         next: None,
//     };

//     let list = List {
//         head: Some(Box::new(node1)),
//     };

//     // for x in list {
//     //     println!("{}", x);
//     // }

//     // node1.next = Some(Box::new(node2));

// }

fn main() {
    println!("main");
}