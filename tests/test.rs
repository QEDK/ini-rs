use ini::{ini, inimerge, inistr, iniwrite, iniwrites};

#[test]
fn load_and_read() {
    let map1 = ini!(safe "tests/test.ini");
    let (map2, map3) = ini!("tests/test.ini", "tests/test.ini");
    assert_eq!(map2, map3);
    assert_eq!(map3, map1.clone().unwrap());
    let instring = "[values]
	key = value

		[VALUES2]
			key2=value2

	[section]
	key = 42";
    let map4 = inistr!(safe instring);
    let owned = instring.to_owned();
    let (map5, map6) = inistr!(&owned, instring);
    assert_eq!(map5, map6);
    assert_eq!(map5, map4.clone().unwrap());
    assert_eq!(map2, map5);
    assert_eq!(map1.unwrap()["section"]["key"].clone().unwrap(), "42");
    assert!(ini!(safe "tests/missing.ini").is_err());
    assert!(inistr!(safe "[unclosed").is_err());
}

#[test]
fn case_sensitive() {
    let map = ini!(cs "tests/test.ini");
    assert!(map.contains_key("VALUES2"));
    assert!(!map.contains_key("values2"));
    assert_eq!(map, ini!(safe cs "tests/test.ini").unwrap());
    assert_eq!(
        map,
        inistr!(cs std::fs::read_to_string("tests/test.ini").unwrap())
    );
    assert_eq!(
        map,
        inistr!(safe cs std::fs::read_to_string("tests/test.ini").unwrap()).unwrap()
    );
}

#[test]
fn merge() {
    let map = inimerge!("tests/test.ini", "tests/override.ini");
    assert_eq!(map["section"]["key"].clone().unwrap(), "43");
    assert_eq!(map["values"]["key"].clone().unwrap(), "value");
    assert_eq!(map["extra"]["new"].clone().unwrap(), "yes");
    assert_eq!(
        map,
        inimerge!(safe "tests/test.ini", "tests/override.ini").unwrap()
    );
    assert!(inimerge!(cs "tests/test.ini", "tests/override.ini").contains_key("VALUES2"));
    assert!(inimerge!(safe cs "tests/test.ini", "tests/missing.ini").is_err());
    assert_eq!(inimerge!("tests/test.ini"), ini!("tests/test.ini"));
}

#[test]
fn write() {
    let map = inistr!(
        "[section]
	key = value
	empty =
	none"
    );
    let output = iniwrites!(map);
    assert_eq!(inistr!(&output), map);

    let path = std::env::temp_dir().join("ini-rs-test-write.ini");
    iniwrite!(&path, map);
    assert_eq!(ini!(&path), map);
    iniwrite!(safe & path, &map).unwrap();
    assert_eq!(ini!(&path), map);
    assert!(iniwrite!(safe "tests/no/such/dir.ini", map).is_err());
}
