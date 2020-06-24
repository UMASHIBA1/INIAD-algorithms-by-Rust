pub trait ListMethods {
    type ELEMENT;
    fn new() -> Self;
    fn list_print(&self) -> (); //リスト要素表示
    fn list_insert(&mut self, pos: usize, value: Self::ELEMENT) -> ();
    fn list_delete(&self, pos: usize) -> ();
    fn list_first(&self) -> usize;
    fn list_last(&self) -> usize;
    fn list_next(&self, pos: usize) -> usize;
    fn list_previous(&self, pos: usize) -> usize;
    fn list_get(&self, pos: usize) -> Result<Self::ELEMENT, &str>;
    fn list_index(&self, pos: &Self::ELEMENT) -> usize;
}
