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

### Installation
You can install this easily via `cargo` by including it in your `Cargo.toml` file like:
```TOML
[dependencies]
ini = "1.0.0"
```

### The `ini!` macro
The `ini!` macro allows you to simply get a hashmap for a list of files. It is planned to provide shell expansion and file-writing in the future:
```ignore,rust
#[macro_use]
extern crate ini;

fn main() {
  let map = ini!("...path/to/file");
  // Proceed to use normal HashMap functions on the map:
  let val = map["section"]["key"].clone().unwrap();
  // The type of the map is HashMap<String, HashMap<String, Option<String>>>

  // To load multiple files, just do:
  let (map1, map2, map3) = ini!("path/to/file1", "path/to/file2", "path/to/file3");
  // Each map is a cloned hashmap with no relation to other ones
}
```
If loading a file fails or the parser is unable to parse the file, the code will `panic` with an appropriate error. In case, you want to handle this
gracefully, it's recommended you use the `safe` metavariable instead. This will make sure your code does not panic and instead exists as a
`Result<HashMap, String>` type and let you deal with errors gracefully.
```ignore,rust
let map = ini!(safe "...path/to/file");
// Proceed to use normal HashMap functions on the map:
let val = map.unwrap()["section"]["key"].clone().unwrap();
// Note the extra unwrap here, which is required because our HashMap is inside a Result type.
```

## Supported datatypes
`configparser` does not guess the datatype of values in configuration files and stores everything as strings, same applies to `ini`. If you need getters that parse the values for you, you might want to use `configparser`. You can ofcourse just choose to parse the string values yourself.
```ignore,rust
let my_string = map["section"]["key"].clone().unwrap();
let my_int = my_string.parse::<i32>().unwrap();
```

## Supported `ini` file structure
A configuration file can consist of sections, each led by a `[section-name]` header, followed by key-value entries separated by a `=`. By default, section names and key names are case-insensitive. All leading and trailing whitespace is removed from stored keys, values and section names.
Key values can be omitted, in which case the key-value delimiter (`=`) may also be left out (but this is different from putting a delimiter, we'll
explain it later). Key-value pairs or section headers cannot span multiple lines.
Owing to how ini files usually are, this means that `[`, `]` and `=` are special symbols (this crate will allow you to use `]` sparingly).

Let's take for example:
```INI
[section headers are case-insensitive]
[   section headers are case-insensitive    ]
are the section headers above same? = yes
sectionheaders_and_keysarestored_in_lowercase? = yes
keys_are_also_case_insensitive = Values are case sensitive
spaces in keys=allowed
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
The only bit of magic the API does is the section-less properties are put in a section called "default". It is planned to allow configuring this variable.
*/
pub use configparser;
use std::collections::HashMap;

///The `ini!` macro allows you to simply get a hashmap for a list of files. It is planned to provide shell expansion and file-writing in the future:
///```ignore,rust
///#[macro_use]
///extern crate ini;
///
///fn main() {
///  let map = ini!("...path/to/file");
///  // Proceed to use normal HashMap functions on the map:
///  let val = map["section"]["key"].clone().unwrap();
///  // The type of the map is HashMap<String, HashMap<String, Option<String>>>
///
///  // To load multiple files, just do:
///  let (map1, map2, map3) = ini!("path/to/file1", "path/to/file2", "path/to/file3");
///  // Each map is a cloned hashmap with no relation to other ones
///}
///```
///If loading a file fails or the parser is unable to parse the file, the code will `panic` with an appropriate error. In case, you want to handle this
///gracefully, it's recommended you use the `safe` metavariable instead. This will make sure your code does not panic and instead exists as a
///`Result<HashMap, String>` type and let you deal with errors gracefully.
///```ignore,rust
///let map = ini!(safe "...path/to/file");
///// Proceed to use normal HashMap functions on the map:
///let val = map.unwrap()["section"]["key"].clone().unwrap();
///// Note the extra unwrap here, which is required because our HashMap is inside a Result type.
///```
#[macro_export]
macro_rules! ini {
	{$($path:literal),+} => {{
		($($crate::macro_load($path)),+)
	}};
	{safe $($path:literal),+} => {{
		($($crate::macro_safe_load($path)),+)
	}};
}

pub fn macro_load(path: &str) -> HashMap<String, HashMap<String, Option<String>>> {
	let mut config = configparser::ini::Ini::new();
	match config.load(path) {
		Err(why) => panic!("{}", why),
		Ok(map) => map
	}
}

pub fn macro_safe_load(path: &str) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
	let mut config = configparser::ini::Ini::new();
	config.load(path)
}