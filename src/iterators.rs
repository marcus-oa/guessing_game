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