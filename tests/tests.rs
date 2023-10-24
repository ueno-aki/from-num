use from_num::from_num;

#[derive(Debug,PartialEq)]
#[from_num(i8,u64,usize)]
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
    assert_eq!(Planet::Mercury,Planet::from_i8(0 as i8).unwrap());
    println!("{:?}",Planet::from_u64(12 as u64));
    println!("{:?}",Planet::from_usize(0xff))
}