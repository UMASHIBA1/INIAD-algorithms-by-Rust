extern crate lists;

use lists::array_list::array_list::ArrayList;
use lists::methods::methods::ListMethods;

fn main() {
    let mut list1 = ArrayList::new();

    // list1.list_insert(1, 6);
    // list1.list_insert(2, 7);
    for i in 0..6 {
        list1.list_insert(i, i as i64);
    }

    // list1.list_insert(4, 5);
    // list1.list_insert(1, 33);

    list1.list_delete(7);
    list1.list_delete(3);

    list1.list_print();

    println!("listfirst: {}", list1.list_first());

    let index0_result = list1.list_retrieve(0);
    match index0_result {
        Ok(n) => println!("index 0 is {}", n),
        Err(err) => println!("{}", err),
    }
}
