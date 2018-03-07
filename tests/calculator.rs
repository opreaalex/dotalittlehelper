extern crate dotalittlehelper;

use dotalittlehelper::calculator::*;

#[test]
fn test_calculate_add() {
    let calc = Calculation::new(
        Operation::Add,
        vec!(3000, 1500, 4500, 1),
        String::from("test_add")
    );

    let expected = ("test_add", 9001);
    let result = &calculate(vec!(calc))[0];
    
    assert_eq!(expected.0, result.0);
    assert_eq!(expected.1, result.1);
}

#[test]
fn test_calculate_divide() {
    let calc = Calculation::new(
        Operation::Divide,
        vec!(450, 5),
        String::from("test_divide")
    );

    let expected = ("test_divide", 90);
    let result = &calculate(vec!(calc))[0];

    assert_eq!(expected.0, result.0);
    assert_eq!(expected.1, result.1);
}

#[test]
fn test_calculate_multiply() {
    let calc = Calculation::new(
        Operation::Multiply,
        vec!(12, 12, 10),
        String::from("test_multiply")
    );

    let expected = ("test_multiply", 1440);
    let result = &calculate(vec!(calc))[0];

    assert_eq!(expected.0, result.0);
    assert_eq!(expected.1, result.1);
}


#[test]
fn test_calculate_subtract() {
    let calc = Calculation::new(
        Operation::Subtract,
        vec!(55, 12, 3),
        String::from("test_subtract")
    );

    let expected = ("test_subtract", 40);
    let result = &calculate(vec!(calc))[0];

    assert_eq!(expected.0, result.0);
    assert_eq!(expected.1, result.1);
}

