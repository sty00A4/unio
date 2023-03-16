use super::*;

#[test]
fn test_speed() {
    let length = meter!(20.);
    let time = second!(2.);
    let speed = length / time;
    assert_eq!(speed, m_pro_s!(10.));
}

#[test]
fn test_square_meter() {
    let width = meter!(20);
    let height = meter!(10);
    let area = width * height;
    assert_eq!(area, unit!(200, unit_pow!(native!(Meter), 2)));
}