use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    in_edges: HashSet<Rc<Pin<Node<T>>>>,
    out_edges: HashSet<Rc<Pin<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            in_edges: HashSet::new(),
            out_edges: HashSet::new(),
        }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn add_out_edge(&mut self, dest: Rc<Node<T>>) {
        // self.out_edges.insert(dest);
    }

    pub fn add_in_edge(&mut self, source: Rc<Node<T>>) {
        // self.in_edges.insert(source);
    }

    pub fn remove_out_edge(&mut self, dest: &Rc<Node<T>>) {
        // self.out_edges.remove(dest);
    }
}

impl<T> Hash for Node<T>
where
    T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}

impl<T> PartialEq for Node<T>
where
    T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        let value = self.value();
        let other = other.value();

        value.eq(&other)
    }
}

impl<T> Eq for Node<T> where T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static {}
