#[macro_use]
extern crate derive_new;
use std::collections::LinkedList;


#[derive(Debug, Clone, new)]
pub struct Monkey {
    pub id: usize,
    pub items: LinkedList<usize>,
    pub operation: (String, String),
    pub test: usize,
    pub if_true: usize,
    pub if_false: usize,
    pub inspections: i32,
}

impl Monkey {
    pub fn add_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    fn pop_item(&mut self) -> usize {
        return self.items.pop_front().unwrap();
    }
    pub fn inspect(&mut self, divide: bool) -> LinkedList<(usize, usize)> {
        let mut result = LinkedList::new();
        for _ in 0..self.items.len() {
            self.inspections += 1;
            let item = self.pop_item();

            let mut new_worry = item.clone();
            let other = if self.operation.1.parse::<usize>().is_ok() {
                self.operation.1.parse::<usize>().unwrap()
            } else {
                new_worry.clone()
            };

            new_worry = if self.operation.0 == "*" {
                new_worry * other
            } else {
                new_worry + other
            };
            if divide {
                new_worry /= 3;
            }
            
            if new_worry % self.test == 0 {
                result.push_front((self.if_true, new_worry));
            } else {
                result.push_front((self.if_false, new_worry));
            }
        }
        return result;
    }
}
