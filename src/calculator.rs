
pub struct Calculation {
    operation: Operation,
    operands:  Vec<i32>,
    id:        String
}

pub enum Operation {
    Add,
    Divide,
    Multiply,
    Subtract
}

impl Calculation {
    pub fn new(operation: Operation,
               operands:  Vec<i32>,
               id:        String) -> Calculation {
        Calculation{
            operation: operation,
            operands: operands,
            id: id
        }
    }
}

pub fn calculate(calculations: Vec<Calculation>) -> Vec<(String, i32)> {
    let mut results: Vec<(String, i32)> = Vec::new();

    for calc in calculations {
        let mut result = 0;

        for (i, op) in calc.operands.iter().enumerate() {
            if i == 0 {
                result += op;
            } else {
                match calc.operation {
                    Operation::Add      => result += op,
                    Operation::Divide   => result /= op,
                    Operation::Multiply => result *= op,
                    Operation::Subtract => result -= op
                }
            }
        }

        results.push((calc.id, result));
    }

    results
}

