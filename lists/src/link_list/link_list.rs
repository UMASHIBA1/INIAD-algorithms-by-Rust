use crate::methods::methods::ListMethod;

pub struct LinkList<T> {
	value: Box<T>,
	next: Box<LinkList>,
}

impl ListMethod for LinkList {
	type ELEMENT = Self::T;
	type POSITION = 
}

