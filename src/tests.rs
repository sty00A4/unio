use super::*;

#[test]
fn test_speed() {
    let length = meter!(20.);
    let time = second!(2.);
    let speed = length / time;
    assert_eq!(speed, m_pro_s!(10.));
}

#[test]
fn test_area() {
    let width = meter!(20);
    let height = meter!(10);
    let area = width * height;
    assert_eq!(area, area!(200));
}

#[test]
fn test_volume() {
    let width = meter!(20);
    let height = meter!(10);
    let depth = meter!(30);
    let vol = width * height * depth;
    assert_eq!(vol, volume!(6000));
}