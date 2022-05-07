#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct ListIterator<'a, T> {
    cur: &'a List<T>
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.cur {
            List::Cons(ref val, next) => {
                // ここの型があわない
                self.cur = next.as_ref();
                Some(val)
            }
            List::Nil => None,
        }
    }
}


impl<'a, T> List<T> {
    fn iter(&self) -> ListIterator<T> {
        ListIterator{cur: self}
    }
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    for i in list.iter() {
        println!("{:?}", &i);
    }
}
