#[derive(Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("missing x")?
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        let y = parts
            .next()
            .ok_or("missing y")?
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, greedy_enum::FromStr, PartialEq)]
enum Vec {
    Point(Point),
    Num(f64),
}

#[test]
fn test() {
    assert_eq!(
        "1.0,2.0".parse::<Vec>().unwrap(),
        Vec::Point(Point { x: 1.0, y: 2.0 })
    );
    assert_eq!("3.0".parse::<Vec>().unwrap(), Vec::Num(3.0));
    assert!("test".parse::<Vec>().is_err());
}
