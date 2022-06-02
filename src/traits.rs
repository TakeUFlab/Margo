use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::token::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

pub trait Hashing
where
    Self: Hash,
{
    fn hashing(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T> Spanned for (T, Span) {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl<T> Hashing for T where T: Hash {}
