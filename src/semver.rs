use regex::Regex;

static SEMVER_REGEX: &str = r"^v?(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$";

enum SemverValue<'a> {
    Number(u32),
    String(&'a str),
}

fn parse_type(semver: &str) -> (&str, SemverValue) {
    match semver.parse::<u32>() {
        Ok(num) => ("number", SemverValue::Number(num)),
        Err(_) => ("string", SemverValue::String(semver)),
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Semver<'a> {
    pub major: &'a str,
    pub minor: &'a str,
    pub patch: &'a str,
    pub prerelease: Option<&'a str>,
    pub buildmetadata: Option<&'a str>,
}

impl<'a> Semver<'a> {
    pub fn new(
        major: Option<&'a str>,
        minor: Option<&'a str>,
        patch: Option<&'a str>,
        prerelease: Option<&'a str>,
        buildmetadata: Option<&'a str>,
    ) -> Self {
        Self {
            major: major.unwrap_or("0"),
            minor: minor.unwrap_or("0"),
            patch: patch.unwrap_or("0"),
            prerelease: prerelease,
            buildmetadata: buildmetadata,
        }
    }
}

pub fn semver_regex(semver: &str) -> Semver {
    let re = Regex::new(SEMVER_REGEX).unwrap();
    let captures = re.captures(semver).unwrap();

    return Semver::new(
        captures.name("major").map(|c| c.as_str()),
        captures.name("minor").map(|c| c.as_str()),
        captures.name("patch").map(|c| c.as_str()),
        captures.name("prerelease").map(|c| c.as_str()),
        captures.name("buildmetadata").map(|c| c.as_str()),
    );
}

fn compare_version(left: Option<&str>, right: Option<&str>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            let left_type = parse_type(left);
            let right_type = parse_type(right);

            match (left_type.1, right_type.1) {
                (SemverValue::Number(left), SemverValue::Number(right)) => return left < right,
                (SemverValue::String(left), SemverValue::String(right)) => {
                    return left.cmp(right) == std::cmp::Ordering::Greater
                }
                _ => return false,
            }
        }

        (Some(_), None) => return false,
        (None, Some(_)) => return true,
        (None, None) => return false,
    }
}

pub fn semver_compare(left: &str, right: &str, asc: bool) -> bool {
    let left_semver = semver_regex(if asc { left } else { right });
    let right_semver = semver_regex(if asc { right } else { left });

    if left_semver.major != right_semver.major {
        return compare_version(Some(left_semver.major), Some(right_semver.major));
    } else if left_semver.minor != right_semver.minor {
        return compare_version(Some(left_semver.minor), Some(right_semver.minor));
    } else if left_semver.patch != right_semver.patch {
        return compare_version(Some(left_semver.patch), Some(right_semver.patch));
    } else if left_semver.prerelease != right_semver.prerelease {
        return compare_version(left_semver.prerelease, right_semver.prerelease);
    } else if left_semver.buildmetadata != right_semver.buildmetadata {
        return compare_version(left_semver.buildmetadata, right_semver.buildmetadata);
    }

    false
}
