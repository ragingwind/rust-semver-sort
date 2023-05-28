use semver_sort::{
    semver::semver_compare,
    semver::semver_regex,
    semver::Semver, semver_sort,
};

fn create_semver<'a>(
    major: &'a str,
    minor: &'a str,
    patch: &'a str,
    prerelease: Option<&'a str>,
    buildmetadata: Option<&'a str>,
) -> Semver<'a> {
    Semver {
        major: major,
        minor: minor,
        patch: patch,
        prerelease: prerelease,
        buildmetadata: buildmetadata,
    }
}

#[test]
fn test_semver_regex() {
    // tests from https://github.com/sindresorhus/semver-regex/blob/main/test.js
    assert_eq!(
        semver_regex("0.0.0"),
        create_semver("0", "0", "0", None, None)
    );
    assert_eq!(
        semver_regex("v1.0.0"),
        create_semver("1", "0", "0", None, None)
    );
    assert_eq!(
        semver_regex("0.0.0-foo"),
        create_semver("0", "0", "0", Some("foo"), None)
    );
    assert_eq!(
        semver_regex("0.0.0-foo"),
        create_semver("0", "0", "0", Some("foo"), None)
    );
    assert_eq!(
        semver_regex("0.0.0-foo-bar-baz"),
        create_semver("0", "0", "0", Some("foo-bar-baz"), None)
    );
    assert_eq!(
        semver_regex("1.2.3-4"),
        create_semver("1", "2", "3", Some("4"), None)
    );
    assert_eq!(
        semver_regex("2.7.2+asdf"),
        create_semver("2", "7", "2", None, Some("asdf"))
    );
    assert_eq!(
        semver_regex("1.2.3-a.b.c.10.d.5"),
        create_semver("1", "2", "3", Some("a.b.c.10.d.5"), None)
    );
    assert_eq!(
        semver_regex("2.7.2-foo+bar"),
        create_semver("2", "7", "2", Some("foo"), Some("bar"))
    );
    assert_eq!(
        semver_regex("1.2.3-alpha.10.beta"),
        create_semver("1", "2", "3", Some("alpha.10.beta"), None)
    );
    assert_eq!(
        semver_regex("1.2.3-alpha.10.beta+build.unicorn.rainbow"),
        create_semver(
            "1",
            "2",
            "3",
            Some("alpha.10.beta"),
            Some("build.unicorn.rainbow")
        )
    );
    assert_eq!(
        semver_regex("99999.99999.99999"),
        create_semver("99999", "99999", "99999", None, None)
    );
    assert_eq!(
        semver_regex("0.0.4"),
        create_semver("0", "0", "4", None, None)
    );
    assert_eq!(
        semver_regex("1.2.3"),
        create_semver("1", "2", "3", None, None)
    );
    assert_eq!(
        semver_regex("10.20.30"),
        create_semver("10", "20", "30", None, None)
    );
    assert_eq!(
        semver_regex("1.1.2-prerelease+meta"),
        create_semver("1", "1", "2", Some("prerelease"), Some("meta"))
    );
    assert_eq!(
        semver_regex("1.1.2+meta"),
        create_semver("1", "1", "2", None, Some("meta"))
    );
    assert_eq!(
        semver_regex("1.1.2+meta-valid"),
        create_semver("1", "1", "2", None, Some("meta-valid"))
    );
    assert_eq!(
        semver_regex("1.0.0-alpha"),
        create_semver("1", "0", "0", Some("alpha"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-beta"),
        create_semver("1", "0", "0", Some("beta"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha.beta"),
        create_semver("1", "0", "0", Some("alpha.beta"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha.beta.1"),
        create_semver("1", "0", "0", Some("alpha.beta.1"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha.1"),
        create_semver("1", "0", "0", Some("alpha.1"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha0.valid"),
        create_semver("1", "0", "0", Some("alpha0.valid"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha.va1id"),
        create_semver("1", "0", "0", Some("alpha.va1id"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha.0valid"),
        create_semver("1", "0", "0", Some("alpha.0valid"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha-a.b-c-somethinglong+build.1-aef.1-its-okay"),
        create_semver(
            "1",
            "0",
            "0",
            Some("alpha-a.b-c-somethinglong"),
            Some("build.1-aef.1-its-okay")
        )
    );
    assert_eq!(
        semver_regex("1.0.0-rc.1+build.1"),
        create_semver("1", "0", "0", Some("rc.1"), Some("build.1"))
    );
    assert_eq!(
        semver_regex("2.0.0-rc.1+build.123"),
        create_semver("2", "0", "0", Some("rc.1"), Some("build.123"))
    );
    assert_eq!(
        semver_regex("1.2.3-beta"),
        create_semver("1", "2", "3", Some("beta"), None)
    );
    assert_eq!(
        semver_regex("10.2.3-DEV-SNAPSHOT"),
        create_semver("10", "2", "3", Some("DEV-SNAPSHOT"), None)
    );
    assert_eq!(
        semver_regex("1.2.3-SNAPSHOT-123"),
        create_semver("1", "2", "3", Some("SNAPSHOT-123"), None)
    );
    assert_eq!(
        semver_regex("1.0.0"),
        create_semver("1", "0", "0", None, None)
    );
    assert_eq!(
        semver_regex("2.0.0"),
        create_semver("2", "0", "0", None, None)
    );
    assert_eq!(
        semver_regex("1.1.7"),
        create_semver("1", "1", "7", None, None)
    );
    assert_eq!(
        semver_regex("2.0.0+build.1848"),
        create_semver("2", "0", "0", None, Some("build.1848"))
    );
    assert_eq!(
        semver_regex("2.0.1-alpha.1227"),
        create_semver("2", "0", "1", Some("alpha.1227"), None)
    );
    assert_eq!(
        semver_regex("1.0.0-alpha+beta"),
        create_semver("1", "0", "0", Some("alpha"), Some("beta"))
    );
    assert_eq!(
        semver_regex("1.0.0+0.build.1-rc.10000aaa-kk-0.1"),
        create_semver("1", "0", "0", None, Some("0.build.1-rc.10000aaa-kk-0.1"))
    );
    assert_eq!(
        semver_regex("1.0.0-0A.is.legal"),
        create_semver("1", "0", "0", Some("0A.is.legal"), None)
    );
}

#[test]
fn test_semver_main_compare() {
    // Test semver_compare simple
    assert_eq!(semver_compare("0.0.0", "0.0.0", true), false);
    assert_eq!(semver_compare("0.0.1", "0.0.0", true), false);
    assert_eq!(semver_compare("0.1.0", "0.0.1", true), false);
    assert_eq!(semver_compare("1.0.0", "0.1.0", true), false);
    assert_eq!(semver_compare("1.0.1", "1.0.0", true), false);
    assert_eq!(semver_compare("1.1.0", "1.0.1", true), false);
    assert_eq!(semver_compare("2.1.1", "1.1.1", true), false);

    assert_eq!(semver_compare("0.0.0", "0.0.0", false), false);
    assert_eq!(semver_compare("0.0.1", "0.0.0", false), true);
    assert_eq!(semver_compare("0.1.0", "0.0.1", false), true);
    assert_eq!(semver_compare("1.0.0", "0.1.0", false), true);
    assert_eq!(semver_compare("1.0.1", "1.0.0", false), true);
    assert_eq!(semver_compare("1.1.0", "1.0.1", false), true);
    assert_eq!(semver_compare("2.1.1", "1.1.1", false), true);

    // Test semver_compare big number
    assert_eq!(semver_compare("0.0.999", "0.0.998", true), false);
    assert_eq!(semver_compare("0.999.0", "0.998.0", true), false);
    assert_eq!(semver_compare("999.0.0", "998.0.0", true), false);

    assert_eq!(semver_compare("0.0.999", "0.0.998", false), true);
    assert_eq!(semver_compare("0.999.0", "0.998.0", false), true);
    assert_eq!(semver_compare("999.0.0", "998.0.0", false), true);

    // Test semver_compare multiple
    assert_eq!(semver_compare("999.999.999", "999.999.998", true), false);
    assert_eq!(semver_compare("999.999.1999", "999.999.999", true), false);
    assert_eq!(semver_compare("1999.999.999", "999.999.999", true), false);

    assert_eq!(semver_compare("999.999.999", "999.999.998", false), true);
    assert_eq!(semver_compare("999.999.1999", "999.999.999", false), true);
    assert_eq!(semver_compare("1999.999.999", "999.999.999", false), true);
}

#[test]
fn test_semver_prebuild_compare() {
    assert_eq!(semver_compare("0.0.0-abc", "0.0.0-bcf", true), false);
    assert_eq!(semver_compare("1.2.3-4", "1.2.3-3", true), false);
    assert_eq!(
        semver_compare("1.2.3-alpha.10.alpha", "1.2.3-alpha.10.beta", true),
        false
    );
    assert_eq!(
        semver_compare("2.0.1-alpha.1227", "2.0.1-alpha.1228", true),
        false
    );
    assert_eq!(semver_compare("0.0.0-abc", "0.0.0-bcf", false), true);
    assert_eq!(semver_compare("1.2.3-4", "1.2.3-3", false), true);
    assert_eq!(
        semver_compare("1.2.3-alpha.10.alpha", "1.2.3-alpha.10.beta", false),
        true
    );
    assert_eq!(
        semver_compare("2.0.1-alpha.1227", "2.0.1-alpha.1228", false),
        true
    );
    assert_eq!(semver_compare("0.0.0-abc", "0.0.0-bcf", false), true);
    assert_eq!(semver_compare("1.2.3-4", "1.2.3-3", false), true);
    assert_eq!(
        semver_compare("1.2.3-alpha.10.alpha", "1.2.3-alpha.10.beta", false),
        true
    );
    assert_eq!(
        semver_compare("1.0.0-rc.1+build.1", "1.0.0-alpha.1227", false),
        false
    );
    assert_eq!(
        semver_compare("2.0.0+build.1848", "2.0.0-alpha.1227", false),
        false
    );
}

#[test]
fn test_semver_sort() {
    let mut semvers = vec![
        "1.0.0-alpha.1",
        "1.0.0-alpha",
        "1.0.0-alpha.beta",
        "1.0.0-beta",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-rc.1",
        "1.0.0",
    ];

    let result = vec![
        "1.0.0",
        "1.0.0-rc.1",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-beta",
        "1.0.0-alpha.beta",
        "1.0.0-alpha.1",
        "1.0.0-alpha",
    ];

    semver_sort(&mut semvers, true);

    for i in 0..semvers.len() {
        assert_eq!(semvers[i], result[i]);
    }

    let mut s = vec![
        "0.0.4-abc",
        "0.0.3-abc",
        "0.0.1-abc",
        "0.0.2-abc",
        "0.0.0-abc",
    ];

    semver_sort(&mut s, true);
}
