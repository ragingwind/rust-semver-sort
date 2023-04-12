use semver_sort::sort;

#[test]
fn test() {
    let result = sort(2, 2);
    assert_eq!(result, 4);
}