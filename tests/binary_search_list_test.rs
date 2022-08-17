use am_i_algo_yet::binary_search_list::BinarySearchList;

#[test]
fn test_binary_search_list() {
    let foo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    assert_eq!(BinarySearchList::binary_search_list(&foo, 69), true);
    assert_eq!(BinarySearchList::binary_search_list(&foo, 1336), false);
    assert_eq!(BinarySearchList::binary_search_list(&foo, 69420), true);
    assert_eq!(BinarySearchList::binary_search_list(&foo, 69421), false);
    assert_eq!(BinarySearchList::binary_search_list(&foo, 1), true);
    assert_eq!(BinarySearchList::binary_search_list(&foo, 0), false);
}
