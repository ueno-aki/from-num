use from_num::from_num;

#[derive(Debug,PartialEq)]
#[from_num(i8,u64)]
enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter = 0b1000,
    Saturn,
    Uranus = 0xff,
    Neptune
}

#[test]
fn defalut_enum() {
    assert_eq!(Planet::Earth,Planet::from(2 as i8));
    assert_eq!(Planet::Jupiter,Planet::from(0b1000 as i8));
    assert_eq!(Planet::Neptune,Planet::from(256 as u64));
}