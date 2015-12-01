use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct CloneVecIter<T>
where T: Clone {
    vec: Rc<Vec<T>>,
    off: usize,
}

impl<T> CloneVecIter<T>
where T: Clone {
    pub fn new(vec: Vec<T>) -> Self {
        CloneVecIter {
            vec: Rc::new(vec),
            off: 0,
        }
    }
}

impl<T> Iterator for CloneVecIter<T>
where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let e = self.vec.get(self.off).map(Clone::clone);
        match e {
            Some(e) => {
                self.off += 1;
                Some(e)
            },
            None => None
        }
    }
}
