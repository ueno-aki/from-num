use anyhow::Result;
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
fn defalut_enum() -> Result<()>{
    assert_eq!(Planet::Mercury,Planet::from_i8(0 as i8)?);
    assert_eq!(Planet::Saturn,Planet::from_usize(0b1001 as usize)?);
    Ok(())
}