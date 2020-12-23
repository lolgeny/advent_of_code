use std::ops::{Index, IndexMut};
use std::slice::Iter;

/// GrowVec is an extension of [`Vec`](std::alloc::Vec) which grows
#[derive(Clone)]
pub struct GrowVec<T: Clone> {
    default: T,
    vec: Vec<T>
}
impl<T: Clone> GrowVec<T> {
    fn new(default: T) -> Self {
        Self {vec: vec![], default}
    }
    fn from(vec: Vec<T>, default: T) -> Self {
        Self {vec, default}
    }
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn iter(&self) -> Iter<T> {
        self.vec.iter()
    }
    fn push(&mut self, value: T) {
        self.vec.push(value)
    }
}
impl<T: Clone> Index<usize> for GrowVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.vec.len() {
            &self.default
        } else {
            &self.vec[index]
        }
    }
}
impl<T: Clone> IndexMut<usize> for GrowVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let default = self.default.clone();
        if self.len() <= index {self.vec.resize_with(index, || default.clone())}
        &mut self.vec[index]
    }
}

#[derive(Clone)]
pub struct Grid<T: Clone> {
    default: T,
    rows: GrowVec<GrowVec<T>>
}
impl<T: Clone> Grid<T> {
    fn new(default: T) -> Self {
        Self {
            default: default.clone(),
            rows: GrowVec::new(GrowVec::new(default))
        }
    }
}