/*!
This crate provides the `ini!` macro which implements a basic configuration language which provides a structure similar to what’s found in Windows' `ini` files. You can use this to write Rust programs which can be customized by end users easily.

This is a simple macro utility built on top of `configparser` with no other dependencies built on Rust. For more advanced functions, you should use the [configparser](https://crates.io/crates/configparser) crate.

## Quick Start

A basic `ini`-syntax file (we say ini-syntax files because the files don't need to be necessarily `*.ini`) looks like this:
```INI
[DEFAULT]
key1 = value1
pizzatime = yes
cost = 9

[topsecrets]
nuclear launch codes = topsecret

[github.com]
User = QEDK
```
Essentially, the syntax consists of sections, each of which can which contains keys with values.

### The `ini!` macro
The `ini!` macro allows you to simply get a hashmap of type [`IniMap`] for a list of files:
```rust
use ini::ini;

let map = ini!("tests/test.ini");
// Proceed to use normal HashMap functions on the map:
let val = map["section"]["key"].clone().unwrap();
assert_eq!(val, "42");

// To load multiple files, just do:
let (map1, map2) = ini!("tests/test.ini", "tests/test.ini");
// Each map is a cloned hashmap with no relation to other ones
assert_eq!(map1, map2);
```
If loading a file fails or the parser is unable to parse the file, the code will `panic` with an appropriate error. In case, you want to handle this
gracefully, it's recommended you use the `safe` metavariable instead. This will make sure your code does not panic and instead exists as a
`Result<IniMap, String>` type and let you deal with errors gracefully.
```rust
use ini::ini;

let map = ini!(safe "tests/test.ini");
// Proceed to use normal HashMap functions on the map:
let val = map.unwrap()["section"]["key"].clone().unwrap();
// Note the extra unwrap here, which is required because our HashMap is inside a Result type.
assert_eq!(val, "42");
```
Section and key names are lowercased by default. Add the `cs` metavariable to keep them case-sensitive. It can be combined with `safe`:
```rust
use ini::ini;

let map = ini!(cs "tests/test.ini");
assert!(map.contains_key("VALUES2"));
let map = ini!(safe cs "tests/test.ini").unwrap();
assert!(map.contains_key("VALUES2"));
```

### The `inistr!` macro
The `inistr!` macro works the same way as `ini!` but parses strings instead of files, and accepts the same `safe` and `cs` metavariables:
```rust
use ini::inistr;

let configstring = "[section]
  key = value
  top = secret";
let map = inistr!(configstring);
assert_eq!(map["section"]["top"].clone().unwrap(), "secret");

let (map1, map2) = inistr!(&String::from(configstring), configstring);
assert_eq!(map1, map2);

let map = inistr!(safe cs "[Section]
  Key = value").unwrap();
assert_eq!(map["Section"]["Key"].clone().unwrap(), "value");
```

### The `inimerge!` macro
The `inimerge!` macro loads several files into a single map, in order. Values from later files override earlier ones, which is handy for layering
user configuration on top of defaults. It accepts the same `safe` and `cs` metavariables:
```rust
use ini::inimerge;

let map = inimerge!("tests/test.ini", "tests/override.ini");
assert_eq!(map["section"]["key"].clone().unwrap(), "43"); // overridden
assert_eq!(map["values"]["key"].clone().unwrap(), "value"); // kept from the first file
```

### The `iniwrite!` and `iniwrites!` macros
`iniwrite!` writes a map back to a file with valid ini-syntax, and `iniwrites!` returns the same output as a `String`. `iniwrite!` panics on I/O
errors unless the `safe` metavariable is used, in which case it returns a `std::io::Result<()>`:
```rust
use ini::{inistr, iniwrite, iniwrites};

let map = inistr!("[section]
  key = value");
let output = iniwrites!(map);
assert_eq!(inistr!(&output), map); // round-trips

let path = std::env::temp_dir().join("ini-doctest.ini");
iniwrite!(&path, map);
iniwrite!(safe &path, &map).unwrap();
```

## Supported datatypes
`configparser` does not guess the datatype of values in configuration files and stores everything as strings, same applies to `ini`. If you need getters that parse the values for you, you might want to use the `configparser` crate. You can ofcourse just choose to parse the string values yourself.
```rust
# let map = ini::inistr!("[section]\nkey = 1");
let my_string = map["section"]["key"].clone().unwrap();
let my_int = my_string.parse::<i32>().unwrap();
```

## Map type
Every macro returns an [`IniMap`], a nested `HashMap<String, HashMap<String, Option<String>>>`. Enable the `indexmap` feature to get an
`IndexMap` instead, which preserves the order of sections and keys as they appear in the file:
```TOML
[dependencies]
ini = { version = "2.0.0", features = ["indexmap"] }
```

## Supported `ini` file structure
A configuration file can consist of sections, each led by a `[section-name]` header, followed by key-value entries separated by a `=`. By default, section names and key names are case-insensitive (use `cs` to change that). All leading and trailing whitespace is removed from stored keys, values and section names.
Key values can be omitted, in which case the key-value delimiter (`=`) may also be left out (but this is different from putting a delimiter, we'll
explain it later). You can use comment symbols (`;` and `#` to denote comments). If you want to select custom symbols, use the `configparser` crate.
Keep in mind that key-value pairs or section headers cannot span multiple lines.
Owing to how ini files usually are, this means that `[`, `]`, `=`, ';' and `#` are special symbols (this crate will allow you to use `]` sparingly).

Let's take for example:
```INI
[section headers are case-insensitive]
[   section headers are case-insensitive    ]
are the section headers above same? = yes
sectionheaders_and_keysarestored_in_lowercase? = yes
keys_are_also_case_insensitive = Values are case sensitive
;anything after a comment symbol is ignored
#this is also a comment
spaces in keys=allowed ;and everything before this is still valid!
spaces in values=allowed as well
spaces around the delimiter = also OK


[All values are strings]
values like this= 0000
or this= 0.999
are they treated as numbers? = no
integers, floats and booleans are held as= strings

[value-less?]
a_valueless_key_has_None
this key has an empty string value has Some("") =

    [indented sections]
        can_values_be_as_well = True
        purpose = formatting for readability
        is_this_same     =        yes
            is_this_same=yes
```
An important thing to note is that values with the same keys will get updated, this means that the last inserted key (whether that's a section header
or property key) is the one that remains in the `HashMap`.
The only bit of magic the API does is the section-less properties are put in a section called "default".
*/
pub use configparser;
use configparser::ini::Ini;
use std::path::Path;

/// The nested map returned by every macro in this crate: sections to keys to optional values.
///
/// This is a `HashMap` by default and an `IndexMap` with the `indexmap` feature enabled.
#[cfg(not(feature = "indexmap"))]
pub type IniMap =
    std::collections::HashMap<String, std::collections::HashMap<String, Option<String>>>;
/// The nested map returned by every macro in this crate: sections to keys to optional values.
///
/// This is a `HashMap` by default and an `IndexMap` with the `indexmap` feature enabled.
#[cfg(feature = "indexmap")]
pub type IniMap = indexmap::IndexMap<String, indexmap::IndexMap<String, Option<String>>>;

/// Loads one or more files, returning an [`IniMap`] per file.
///
/// Prefix with `safe` to get a `Result<IniMap, String>` instead of panicking on error, and with `cs`
/// to keep section and key names case-sensitive. Both can be combined as `safe cs`.
/// ```rust
/// use ini::ini;
///
/// let map = ini!("tests/test.ini");
/// let val = map["section"]["key"].clone().unwrap();
/// assert_eq!(val, "42");
///
/// // Each map is a cloned hashmap with no relation to other ones
/// let (map1, map2) = ini!("tests/test.ini", "tests/test.ini");
/// assert_eq!(map1, map2);
///
/// let map = ini!(safe cs "tests/test.ini").unwrap();
/// assert!(map.contains_key("VALUES2"));
/// assert!(ini!(safe "no/such/file.ini").is_err());
/// ```
#[macro_export]
macro_rules! ini {
    {safe cs $($path:expr),+} => {{
        ($($crate::load($path, true)),+)
    }};
    {safe $($path:expr),+} => {{
        ($($crate::load($path, false)),+)
    }};
    {cs $($path:expr),+} => {{
        ($($crate::load($path, true).unwrap_or_else(|why| panic!("{}", why))),+)
    }};
    {$($path:expr),+} => {{
        ($($crate::load($path, false).unwrap_or_else(|why| panic!("{}", why))),+)
    }};
}

/// Parses one or more strings, returning an [`IniMap`] per string.
///
/// Accepts the same `safe` and `cs` prefixes as [`ini!`].
/// ```rust
/// use ini::inistr;
///
/// let configstring = "[section]
///   key = value
///   top = secret";
/// let map = inistr!(configstring);
/// assert_eq!(map["section"]["top"].clone().unwrap(), "secret");
///
/// let (map1, map2) = inistr!(&String::from(configstring), configstring);
/// assert_eq!(map1, map2);
///
/// let map = inistr!(safe cs "[Section]
///   Key = value").unwrap();
/// assert_eq!(map["Section"]["Key"].clone().unwrap(), "value");
/// ```
#[macro_export]
macro_rules! inistr {
    {safe cs $($instring:expr),+} => {{
        ($($crate::read($instring, true)),+)
    }};
    {safe $($instring:expr),+} => {{
        ($($crate::read($instring, false)),+)
    }};
    {cs $($instring:expr),+} => {{
        ($($crate::read($instring, true).unwrap_or_else(|why| panic!("{}", why))),+)
    }};
    {$($instring:expr),+} => {{
        ($($crate::read($instring, false).unwrap_or_else(|why| panic!("{}", why))),+)
    }};
}

/// Loads several files into one [`IniMap`], with later files overriding earlier ones.
///
/// Accepts the same `safe` and `cs` prefixes as [`ini!`].
/// ```rust
/// use ini::inimerge;
///
/// let map = inimerge!("tests/test.ini", "tests/override.ini");
/// assert_eq!(map["section"]["key"].clone().unwrap(), "43");
/// assert_eq!(map["values"]["key"].clone().unwrap(), "value");
/// assert!(inimerge!(safe "tests/test.ini", "no/such/file.ini").is_err());
/// ```
#[macro_export]
macro_rules! inimerge {
    {safe cs $($path:expr),+} => {
        $crate::merge(&[$(::std::path::Path::new(&$path)),+], true)
    };
    {safe $($path:expr),+} => {
        $crate::merge(&[$(::std::path::Path::new(&$path)),+], false)
    };
    {cs $($path:expr),+} => {
        $crate::merge(&[$(::std::path::Path::new(&$path)),+], true).unwrap_or_else(|why| panic!("{}", why))
    };
    {$($path:expr),+} => {
        $crate::merge(&[$(::std::path::Path::new(&$path)),+], false).unwrap_or_else(|why| panic!("{}", why))
    };
}

/// Writes an [`IniMap`] to a file, creating or overwriting it.
///
/// Panics on I/O errors unless prefixed with `safe`, which returns a `std::io::Result<()>`.
/// ```rust
/// use ini::{ini, inistr, iniwrite};
///
/// let map = inistr!("[section]
///   key = value");
/// let path = std::env::temp_dir().join("ini-doctest-write.ini");
/// iniwrite!(&path, map);
/// assert_eq!(ini!(&path), map);
/// assert!(iniwrite!(safe "no/such/dir/file.ini", map).is_err());
/// ```
#[macro_export]
macro_rules! iniwrite {
    {safe $path:expr, $map:expr} => {
        $crate::write($path, &$map)
    };
    {$path:expr, $map:expr} => {
        $crate::write($path, &$map).unwrap_or_else(|why| panic!("{}", why))
    };
}

/// Formats an [`IniMap`] as an ini-syntax `String`.
/// ```rust
/// use ini::{inistr, iniwrites};
///
/// let map = inistr!("[section]
///   key = value");
/// let output = iniwrites!(map);
/// assert!(output.starts_with("[section]"));
/// assert_eq!(inistr!(&output), map); // round-trips
/// ```
#[macro_export]
macro_rules! iniwrites {
    {$map:expr} => {
        $crate::writes(&$map)
    };
}

fn new_ini(cs: bool) -> Ini {
    if cs { Ini::new_cs() } else { Ini::new() }
}

fn from_map(map: &IniMap) -> Ini {
    let mut config = Ini::new();
    *config.get_mut_map() = map.clone();
    config
}

#[doc(hidden)]
pub fn load(path: impl AsRef<Path>, cs: bool) -> Result<IniMap, String> {
    new_ini(cs).load(path)
}

#[doc(hidden)]
pub fn read(input: impl AsRef<str>, cs: bool) -> Result<IniMap, String> {
    new_ini(cs).read(input.as_ref().to_owned())
}

#[doc(hidden)]
pub fn merge(paths: &[&Path], cs: bool) -> Result<IniMap, String> {
    let mut config = new_ini(cs);
    let mut map = IniMap::default();
    for path in paths {
        map = config.load_and_append(path)?;
    }
    Ok(map)
}

#[doc(hidden)]
pub fn write(path: impl AsRef<Path>, map: &IniMap) -> std::io::Result<()> {
    from_map(map).write(path)
}

#[doc(hidden)]
pub fn writes(map: &IniMap) -> String {
    from_map(map).writes()
}
