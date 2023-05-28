use semver::semver_compare;

pub mod semver;

pub fn semver_sort(semvers: &mut Vec<&str>, asc: bool) {
    semvers.sort_by(|a, b| {
        if semver_compare(a, b, asc) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
}