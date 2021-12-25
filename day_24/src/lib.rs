use std::fmt;
use std::hash::Hash;
use std::collections::{HashMap};

// ================================================== STRUCTS ==================================================

type Value = i64;

enum ArgumentType { Dimension, Value }
enum OperationType { Inp, Add, Mul, Div, Mod, Eql}
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Dimension { X, Y, Z, W }

trait Argument {
    fn get_type(&self) -> ArgumentType;
    fn get_value(&self, alu_state: &HashMap<Dimension, Value>) -> Value;
    fn store_result(&self, result: Value, alu_state: HashMap<Dimension, Value>) -> HashMap<Dimension, Value>;
}

struct DimensionArgument {
    dimension:  Dimension
}

struct ValueArgument {
    value:  Value,
}

struct Operation {
    operation_type: OperationType,
    arguments:      Vec<Box<dyn Argument>>,
}

pub struct ArithmeticLogicUnit {
    state:      HashMap<Dimension, Value>,
    operations: Vec<Operation>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Argument for DimensionArgument {
    
    fn get_type(&self) -> ArgumentType { ArgumentType::Dimension }
    fn get_value(&self, alu_state: &HashMap<Dimension, Value>) -> Value { *alu_state.get(&self.dimension).unwrap() }
    fn store_result(&self, result: Value, mut alu_state: HashMap<Dimension, Value>) -> HashMap<Dimension, Value> {
        alu_state.insert(self.dimension, result);
        return alu_state;
    }
}

impl Argument for ValueArgument {

    fn get_type(&self) -> ArgumentType { ArgumentType::Value }
    fn get_value(&self, _alu_state: &HashMap<Dimension, Value>) -> Value { self.value }
    fn store_result(&self, _result: Value, alu_state: HashMap<Dimension, Value>) -> HashMap<Dimension, Value> { alu_state }
}

impl Operation {

    fn new(line: String) -> Operation {

        let mut splitted_line : Vec<&str> = line.split_whitespace()
            .into_iter().collect();

        let operation_str : &str = splitted_line.remove(0);
        let operation : OperationType = match operation_str {
            "inp" => OperationType::Inp,
            "add" => OperationType::Add,
            "mul" => OperationType::Mul,
            "div" => OperationType::Div,
            "mod" => OperationType::Mod,
            "eql" => OperationType::Eql,
            _ => panic!("🚨  Operation code not intrepertable")
        };

        let arguments : Vec<Box<dyn Argument>> = splitted_line.into_iter()
            .map(|argument_info| {
                return match argument_info {
                    "x" => Box::new(DimensionArgument{ dimension: Dimension::X }) as Box<dyn Argument>,
                    "y" => Box::new(DimensionArgument{ dimension: Dimension::Y }) as Box<dyn Argument>,
                    "z" => Box::new(DimensionArgument{ dimension: Dimension::Z }) as Box<dyn Argument>,
                    "w" => Box::new(DimensionArgument{ dimension: Dimension::W }) as Box<dyn Argument>,
                    value => Box::new(ValueArgument{ value: value.parse().unwrap() }) as Box<dyn Argument>,
                };

            }).collect();

        Operation {
            operation_type: operation,
            arguments: arguments,
        }
    }

    fn execute(&self, mut alu_state: HashMap<Dimension, Value>, mut input: Vec<Value>) -> (HashMap<Dimension, Value>, Vec<Value>) {

        let values : Vec<Value> = self.arguments.iter()
            .map(|argument| argument.get_value(&alu_state))
            .collect();

        let result = match self.operation_type {
            OperationType::Inp => input.pop().unwrap(),
            OperationType::Add => values[0] + values[1],
            OperationType::Mul => values[0] * values[1],
            OperationType::Div => (values[0] as f64 / values[1] as f64).floor() as Value,
            OperationType::Mod => values[0] % values[1],
            OperationType::Eql => if values[0] == values[1] { 1 } else { 0 },
        };

        alu_state = self.arguments[0].store_result(result, alu_state);
        return (alu_state, input);
    }
}

impl ArithmeticLogicUnit {

    pub fn new(instructions: Vec<String>) -> ArithmeticLogicUnit {

        let alu_state : HashMap<Dimension, Value> = vec!(
            (Dimension::X, 0),
            (Dimension::Y, 0),
            (Dimension::Z, 0),
            (Dimension::W, 0)
        ).into_iter().collect();

        let instructions : Vec<Operation> = instructions.into_iter()
            .map(|instruction| Operation::new(instruction))
            .collect();

        ArithmeticLogicUnit {
            state: alu_state,
            operations: instructions,
        }
    }

    pub fn mannual_fix_dimension(&mut self, dimension: Dimension, value: Value) {
        self.state.insert(dimension, value);
    }

    pub fn get_dimension(&mut self, dimension: Dimension) -> Value { *self.state.get(&dimension).unwrap() }

    pub fn process_input(&mut self, mut input: Vec<Value>) {

        for instruction in self.operations.iter() {
            let result = instruction.execute(self.state.clone(), input);
            self.state = result.0;
            input = result.1;
        }
    }
}

impl fmt::Display for ArithmeticLogicUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let dimensions : Vec<Dimension> = vec!(Dimension::X, Dimension::Y, Dimension::Z, Dimension::W);

        let dimensions_info : Vec<String> = dimensions.into_iter()
            .map(|dimension| format!("{:#?}: {: >14}", dimension, self.state.get(&dimension).unwrap()))
            .collect();

        return write!(f, "{}", dimensions_info.join(" | "));
    }
}