#[macro_use]
extern crate ini;

#[test]
fn main() {
	let map1 = ini!(safe "tests/test.ini");
	let (map2, map3) = ini!("tests/test.ini", "tests/test.ini");
	assert_eq!(assert_eq!(map2, map3), assert_eq!(map3, map1.clone().unwrap()));
	assert_eq!(map1.unwrap()["section"]["key"].clone().unwrap(), "42");
}