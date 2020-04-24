use crate::methods::methods::ListMethods;

const INITIAL_SIZE: usize = 10;

pub struct ArrayList {
    elements: Box<[i64; INITIAL_SIZE]>,
    length: usize,
    max_length: usize,
}


impl ListMethods for ArrayList {
    type ELEMENT = i64;
    fn list_print(&self) {
        for i in 0..*(&self.length) {
            println!("{}", &self.elements[i]);
        }
    }
}
