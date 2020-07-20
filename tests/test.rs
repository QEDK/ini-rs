#[macro_use]
extern crate ini;

#[test]
fn main() {
	let map1 = ini!(safe "tests/test.ini");
	let (map2, map3) = ini!("tests/test.ini", "tests/test.ini");
	assert_eq!(assert_eq!(map2, map3), assert_eq!(map3, map1.clone().unwrap()));
	let instring = "[values]
	key = value

		[VALUES2]
			key2=value2

	[section]
	key = 42";
	let map4 = inistr!(safe "[values]
	key = value

		[VALUES2]
			key2=value2

	[section]
	key = 42");
	let (map5, map6) = inistr!(&instring.to_owned(), instring);
	assert_eq!(assert_eq!(map5, map6), assert_eq!(map5, map4.clone().unwrap()));
	assert_eq!(assert_eq!(map2, map5), assert_eq!(map1, map4));
	assert_eq!(map1.unwrap()["section"]["key"].clone().unwrap(), "42");
}