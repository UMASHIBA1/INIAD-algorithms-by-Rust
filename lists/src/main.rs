extern crate lists;

use lists::array_list::array_list::ArrayList;
use lists::methods::methods::ListMethods;

fn main() {
    let mut list1 = ArrayList::new();

    list1.list_insert(0, 5);

    list1.list_print();

    let index0_result = list1.list_get(0);
    match index0_result {
        Ok(n) => println!("index 0 is {}", n),
        Err(err) => println!("{}", err),
    }
}
