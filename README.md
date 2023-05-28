# semver_sort

> Sorts semver strings or numbers. and more util functions

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
semver_sort = "1"
```

## API

### semver_regex

Parse semver string to Semver type

```rust
use semver_sort::{
  semver::semver_regex
};

print!("{:?}", semver_regex("1.2.3-alpha.10.beta"));
// Semver { major: "1", minor: "2", patch: "3", prerelease: Some("alpha.10.beta"), buildmetadata: None }
```

## semver_compare

Compare returns Ordering

```rust
use semver_sort::{
    semver::semver_compare,
    semver::Semver,
};

semver_compare("0.0.0-abc", "0.0.0-abc", true);
// Ordering::Equal

semver_compare("0.0.0-bcd", "0.0.0-abc", true);
// Ordering::Great

semver_compare("0.0.0-abc", "0.0.0-bcd", true); 
// Ordering::Less
```

## semver_sort

Returns sorted semver

```rust
use semver_sort::{
  semver_sort
};

semver_sort(["0.0.4-abc", "0.0.3-abc", "0.0.1-abc", "0.0.2-abc", "0.0.0-abc"]);
// ["0.0.0-abc", "0.0.1-abc", "0.0.2-abc", "0.0.3-abc", "0.0.4-abc"]
```

## License