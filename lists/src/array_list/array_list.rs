use crate::methods::methods::ListMethods;

const INITIAL_SIZE: usize = 10;

pub struct ArrayList {
    elements: Box<[i64; INITIAL_SIZE]>,
    length: usize,
    max_length: usize,
}

impl ListMethods for ArrayList {
    type ELEMENT = i64;

    fn new() -> Self {
        ArrayList {
            elements: Box::new([0; INITIAL_SIZE]),
            length: 0,
            max_length: INITIAL_SIZE,
        }
    }

    fn list_print(&self) {
        println!("run print!");
        for i in 0..self.length {
            println!("{}", &self.elements[i]);
        }
    }

    fn list_insert(&mut self, pos: usize, value: Self::ELEMENT) {
        if pos < 0 || pos > *(&self.length) {
            ()
        }
        // if self.length == self.max_length {

        // }

        for i in ((pos - 1)..self.length).rev() {
            let element_box = &mut self.elements;
            element_box[i] = element_box[i - 1];
        }
        self.elements[pos] = value;
        self.length = self.length + 1;
    }

    fn list_delete(&self, pos: usize) {
        ()
    }

    fn list_first(&self) -> usize {
        1 as usize
    }

    fn list_last(&self) -> usize {
        1 as usize
    }

    fn list_next(&self, pos: usize) -> usize {
        1 as usize
    }

    fn list_previous(&self, pos: usize) -> usize {
        1 as usize
    }

    fn list_get(&self, pos: usize) -> Result<Self::ELEMENT, &str> {
        if self.length == 0 {
            return Err("list does not exist any element");
        }
        if pos >= self.length - 1 {
            return Err("the pos index of elements does not exist");
        }
        Ok(self.elements[pos])
    }

    fn list_index(&self, pos: &Self::ELEMENT) -> usize {
        1 as usize
    }
}
