use std::collections::VecDeque;

#[derive(Debug)]
pub struct Window<T> {
    vec: VecDeque<T>,
    max_size: usize
}

impl<T> Window<T> {
    pub fn new(max_size: usize) -> Window<T> {
        return Window {
            vec: VecDeque::with_capacity(max_size),
            max_size: max_size
        };
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn len(&self) -> usize {
        return self.vec.len();
    }

    /// stacks the vector into the window taking ownership
    /// of items in vector.
    pub fn extend(&mut self, data: Vec<T>) {
        for item in data {
            self.stack(item);
        }
    }

    /// stacks an item into the window remove
    /// the earliest item if the max size has been reached
    pub fn stack(&mut self, data: T) {
        if self.vec.len() >= self.max_size {
            self.vec.pop_front();
        }
        self.vec.push_back(data);
    }

    /// Returns a mutable reference to the internal deque.
    pub fn get_mut(&mut self) -> &mut VecDeque<T> {
        return &mut self.vec;
    }

    /// Returns a reference to the internal deque.
    pub fn get(&self) -> &VecDeque<T> {
        return &self.vec;
    }

    /// Retrieves references to the latest values
    /// up to the specified amount. The returned values could
    /// be less then this amount if there is not enough
    /// in the window. 
    pub fn peek(&self, amount: usize) -> VecDeque<&T> {
        let mut index = 0;
        let mut l: VecDeque<&T> = VecDeque::with_capacity(amount);
        for value in self.vec.iter().rev() {
            l.push_front(value);
            index += 1;
            if index >= amount {
                break;
            }
        }
        return l;
    }

    /// Pops the latests values up to the 
    /// specified amount taking ownership. 
    /// The returned values could be less then 
    /// this amount if there is not enough in the window.
    pub fn pop(&mut self, amount: usize) -> VecDeque<T> {
        let mut list: VecDeque<T> = VecDeque::with_capacity(amount);
        for _ in 0..amount {
            match self.vec.pop_back() {
                Some(value) => list.push_front(value),
                None => break
            };
        }

        return list;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_peek_latest_values() {
        let mut window = Window::<usize>::new(3);
        window.stack(1);
        window.stack(2);
        window.stack(3);
        window.stack(4);

        let mut expected_vec = VecDeque::new();
        expected_vec.push_back(&2);
        expected_vec.push_back(&3);
        expected_vec.push_back(&4);

        assert_eq!(window.peek(3), expected_vec);
        assert_eq!(window.len(), 3);
    }

    #[test]
    fn should_pop_latest_values() {
        let mut window = Window::<usize>::new(3);
        window.stack(1);
        window.stack(2);
        window.stack(3);
        window.stack(4);

        let mut expected_vec = VecDeque::new();
        expected_vec.push_back(2);
        expected_vec.push_back(3);
        expected_vec.push_back(4);

        assert_eq!(window.pop(3), expected_vec);
        assert_eq!(window.len(), 0);
    }
    
    #[test]
    fn should_extend_from_vec() {
        let mut window = Window::<usize>::new(5);

        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        window.extend(vec);

        assert_eq!(window.len(), 5);
    }

    #[test]
    fn should_mutate_data() {
        let mut window = Window::<usize>::new(5);
        let vec = vec![1, 2, 3, 4, 5];
        window.extend(vec);

        for item in window.get_mut() {
            *item += 1;
        }

        let expected_vec = vec![&2, &3, &4, &5, &6];

        assert_eq!(window.peek(5), expected_vec);
    }
}
