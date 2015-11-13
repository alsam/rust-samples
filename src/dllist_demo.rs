// a double linked list
use std::collections::LinkedList;
use std::iter::FromIterator;

type MyList =  LinkedList<Vec<usize>>;

fn my_print(ll: &MyList) {
    fn nested(l:&MyList) {
        // nested functions are allowed
        println!("len of the list {}", l.len());
    }
    nested(&ll);
    println!("list: {:?}",ll);
}

fn main() {
    let mut a = LinkedList::new();
    a.push_back(vec![1,2]);
    a.push_back(vec![7,8,9]);
    a.push_back(vec![777]);

    my_print(&a);

    let vv = vec![ vec![1], vec![3,4,5], vec![2,7]];
    let b = LinkedList::from_iter( vv );

    my_print(&b);

    let c = LinkedList::from_iter( vec![ vec![111], vec![4,5], vec![2,9,7]]  );

    my_print(&c);

}
