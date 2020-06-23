use crate::methods::methods::ListMethods;

const INITIAL_SIZE: usize = 10;

pub struct ArrayList {
    pub elements: Box<[i64; INITIAL_SIZE]>,
    pub length: usize,
    pub max_length: usize,
}

impl ListMethods for ArrayList {
    type ELEMENT = i64;
    fn list_print(&self) {
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
            let elementBox = &mut self.elements;
            elementBox[i] = elementBox[i - 1];
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

    fn list_get(&self, pos: usize) -> Self::ELEMENT {
        if self.elements.is_empty() {
            ()
        }
        if pos < 0 || pos >= self.length {
            ()
        }
        self.elements[pos]
    }

    fn list_index(&self, pos: &Self::ELEMENT) -> usize {
        1 as usize
    }
}
