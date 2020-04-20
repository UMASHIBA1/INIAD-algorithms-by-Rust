trait ListMethod {
    type LIST;
    type ELEMENT;
    fn construct(&self) -> &Self::LIST;
    fn list_free(&self) -> ();
    fn list_print(&self) -> ();
    fn list_insert(&self, usize, &Self::ELEMENT) -> ();
    fn list_delete(&self, usize) -> ();
    fn list_first(&self) -> usize;
    fn list_last(&self) -> usize;
    fn list_next(&self, usize) -> usize;
    fn list_previous(&self, usize) -> usize;
    fn list_get(&self, usize) -> &Self::ELEMENT;
    fn list_index(&self, &Self::ELEMENT) -> usize;
}
