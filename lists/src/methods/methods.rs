pub trait ListMethods {
    type ELEMENT;
    fn list_print(&self) -> ();
    fn list_insert(&self, pos: usize, value: &Self::ELEMENT) -> ();
    fn list_delete(&self, pos: usize) -> ();
    fn list_first(&self) -> usize;
    fn list_last(&self) -> usize;
    fn list_next(&self, pos: usize) -> usize;
    fn list_previous(&self, pos: usize) -> usize;
    fn list_get(&self, pos: usize) -> &Self::ELEMENT;
    fn list_index(&self, pos: &Self::ELEMENT) -> usize;
}
