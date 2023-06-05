use crate::basic_data_structure::array_data_structure;
use crate::linked_list::Node;

mod basic_data_structure;
mod linked_list;

fn main() {
    println!("Hello, world!");
    array_data_structure();

    let mut list = Node::new(1);
    list.append(12);
    list.append(123);
    list.append(70);
    list.append(75);

    let mut current = &list;

    while let Some(next) = &current.next {
        println!("{}", current.data);
        current = next
    }
    println!("{}", current.data)
}
