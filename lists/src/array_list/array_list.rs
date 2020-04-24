const INITIAL_SIZE: usize = 10;

pub struct ArrayList {
    elements: Box<[i64; INITIAL_SIZE]>,
    length: usize,
    max_length: usize,
}


