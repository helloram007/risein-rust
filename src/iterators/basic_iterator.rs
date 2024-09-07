fn main() {
    let my_vec = vec![1,2,3,4,5];

    let mut my_iter = my_vec.iter();

    assert_eq!(my_iter.next(), Some(&1));
    assert_eq!(my_iter.next(), Some(&2));
    assert_eq!(my_iter.next(), Some(&3));
    assert_eq!(my_iter.next(), Some(&4));
    assert_eq!(my_iter.next(), Some(&5));
    assert_eq!(my_iter.next(), None);

}