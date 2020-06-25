pub trait ListMethods {
    type ELEMENT;
    type POSITION;
    fn new() -> Self;
    fn list_print(&self) -> (); //リスト要素表示
    fn list_insert(&mut self, pos: Self::POSITION, value: Self::ELEMENT) -> ();
    fn list_delete(&mut self, pos: Self::POSITION) -> ();
    fn list_first(&self) -> Self::POSITION;
    fn list_last(&self) -> Self::POSITION;
    fn list_next(&self, pos: Self::POSITION) -> Self::POSITION;
    fn list_previous(&self, pos: Self::POSITION) -> Self::POSITION;
    fn list_retrieve(&self, pos: Self::POSITION) -> Result<Self::ELEMENT, &str>; //posの要素を取得
    fn list_get(&self, pos: Self::POSITION) -> Self::POSITION;
}
