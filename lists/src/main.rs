extern crate lists;

use lists::array_list::array_list::ArrayList;
use lists::methods::methods::ListMethods;

fn main() {
    let mut list1 = ArrayList {
        elements: Box::new([1; 10]),
        length: 0,
        max_length: 10,
    };

    println!("{}", list1.list_get(0));

    // list1.list_print();
}
