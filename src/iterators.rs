pub fn iterator_example() {
    let v1 = vec![1, 2, 3];

    // Note: Iterators are lazy in that they don't execute
    // until they are called
    let v1_iter = v1.iter();

    // This for loop take ownership of the iterator created
    // above and this doesn't need a mut instance like in
    // later examples
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // mutability is required here as the operation
    // 'next' changes the internal state of the iterator
    let mut v1_iter = v1.iter();

    // iterators return immutable references to the
    // values they hold
    // Note: we can return owned values with no reference
    // by instantiating with into_iter,
    // for values with mutability we can call iter_mut
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

}

#[test]
// Example of consuming adaptor: a method
// that calls the next function and consumes
// the iterator
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // after the sum call we can't use v1_iter
    // as its ownership has been taken
    let total: i32 = v1_iter.sum();


    assert_eq!(total, 6);
}

#[test]
fn iterator_adaptor_example() {
    let v1 = vec![1, 2, 3];

    // This call is lazy without collect; it won't execute and will produce
    // a warning
    // 'collect' consumes the iterator and collects the results into
    // a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);


}

#[derive(PartialEq, Debug)]
// simple struct to hold shoe data
struct Shoe {
    size: u32,
    style: String,
}

// function which uses a closure and filter to return only a set
// of shoes which pass the bool test
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec! [
            Shoe {
                size: 10,
                style: String:: from("sneaker")
            },
            Shoe {
                size: 13,
                style: String:: from("sandal")
            },            Shoe {
                size: 10,
                style: String:: from("boot")
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String:: from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String:: from("boot")
                },
            ]
        );
    }
}

// Creating our own iterators: Exampled below
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
// Test to show Counters implementation of Iterator
// Will produce a next value up until 5 has been reached
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
// example to show that once you've implemented Iterator
// you will access to the other iterator methods
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum)
}

