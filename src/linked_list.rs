pub(crate) struct Node<T>{
    pub(crate) data: T,
    pub(crate) next: Option<Box<Node<T>>>,
}

impl <T> Node<T>{

    pub(crate) fn new(data: T) -> Self{
        Node {data, next: None }
    }

    pub(crate) fn append(&mut self, data: T){
        match &mut self.next{
            Some(next) => next.append(data),
            None =>{
                let new_node = Box::new(Node::new(data));
                self.next = Some(new_node);
            }
        }
    }
}