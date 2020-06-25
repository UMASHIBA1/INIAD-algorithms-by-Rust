use crate::methods::methods::ListMethods;

const INITIAL_SIZE: usize = 10;


// insert: O(n)
// delete: O(n)
pub struct ArrayList {
    elements: Vec<i64>,
    length: usize,
    max_length: usize,
}

impl ListMethods for ArrayList {
    type ELEMENT = i64;
    type POSITION = usize;

    fn new() -> Self {
        let mut new_list = ArrayList {
            elements: Vec::with_capacity(INITIAL_SIZE),
            length: 0,
            max_length: INITIAL_SIZE,
        };

        for _ in 0..INITIAL_SIZE {
            // elementsの初期値はとりあえず全部0
            new_list.elements.push(0);
        }
        new_list
    }

    fn list_print(&self) {
        for i in 0..self.length {
            println!("{}", &self.elements[i]);
        }
    }

    fn list_insert(&mut self, pos: Self::POSITION, value: Self::ELEMENT) {
        // 指定されたposの位置が存在していない場合
        if pos > self.length {
            ()
        }
        // すでにelmentsが最大長になっている場合、新しくヒープ上に配列を作って内容をコピーする
        if self.length == self.max_length {
            let next_max_length = self.max_length * 2;
            let mut new_elements = Vec::with_capacity(next_max_length);
            for i in 0..self.length {
                new_elements.push(self.elements[i]);
            }
            for i in self.length..next_max_length {
                new_elements.push(0);
            }
            self.elements = new_elements;
            self.max_length = next_max_length;
        }

        // 指定されたposより後ろの要素をずらす
        for i in (pos..self.length).rev() {
            let element_box = &mut self.elements;
            element_box[i + 1] = element_box[i];
        }
        self.elements[pos] = value;
        self.length = self.length + 1;
    }

    fn list_delete(&mut self, pos: Self::POSITION) {
        if pos > self.length {
            return ();
        }
        // 指定されたposより後ろの要素を前にずらす
        for i in pos..self.length {
            self.elements[i] = self.elements[i + 1];
        }
        // 長さ-1
        self.length -= 1;
    }

    fn list_first(&self) -> Self::POSITION {
        0
    }

    fn list_last(&self) -> Self::POSITION {
        self.length - 1
    }

    fn list_next(&self, pos: Self::POSITION) -> Self::POSITION {
        if pos >= self.length {
            return 0;
        }
        pos + 1
    }

    fn list_previous(&self, pos: Self::POSITION) -> Self::POSITION {
        if pos <= 0 {
            return 0;
        }
        pos - 1
    }

    fn list_retrieve(&self, pos: Self::POSITION) -> Result<Self::ELEMENT, &str> {
        if self.length == 0 {
            return Err("list does not exist any element");
        }
        if pos >= self.length - 1 {
            return Err("the pos index of elements does not exist");
        }
        Ok(self.elements[pos])
    }

    fn list_get(&self, pos: Self::POSITION) -> Self::POSITION {
        pos
    }

}
