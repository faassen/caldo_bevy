use std::num::Wrapping;

const GENE_SIZE: usize = 32;
const GENE_AMOUNT: usize = 16;
const PROCESSOR_AMOUNT: usize = 4;
const DATA_STACK_SIZE: usize = 32;
const DATA_STACK_HALF_SIZE: usize = DATA_STACK_SIZE / 2;
const INSTRUCTION_STACK_SIZE: usize = 32;
const INSTRUCTION_STACK_HALF_SIZE: usize = INSTRUCTION_STACK_SIZE / 2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instr {
    Number(i8),
    // Nothing
    Noop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Comparison
    Eq,
    Ne,
    Gt,
    Lt,

    // Boolean
    And,
    Or,
    Not,

    // Stack manipulation
    Dup,
    Drop,
    Swap,
    Over,
    Dup2,
    Drop2,

    // Calling genes
    Call,

    // Cells where we read and write
    Reading,
    Writing,

    // Looping
    SetLoop,
    Loop,

    // Spawn processor
    Spawn, // nr of processor to spawn

    // Environment
    Occupied, // check whether Writing has a cell at all
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Processor {
    active: bool,
    gene_index: usize,
    pc: usize,
    data_stack_index: usize,
    instruction_stack_index: usize,
    data_stack: [i8; DATA_STACK_SIZE],
    instruction_stack: [Instr; INSTRUCTION_STACK_SIZE],
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    genes: [[Instr; GENE_SIZE]; GENE_AMOUNT],
    processors: [Processor; PROCESSOR_AMOUNT],
}

impl Instr {
    fn execute(&self, processor: &mut Processor) {
        match *self {
            Instr::Number(n) => {
                processor.data_push(n);
            }
            Instr::Noop => {}
            Instr::Add => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push(a.wrapping_add(b));
            }
            Instr::Sub => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push(b.wrapping_sub(a));
            }
            Instr::Mul => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push(a.wrapping_mul(b));
            }
            Instr::Div => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                if a == 0 {
                    processor.data_push(0);
                } else {
                    processor.data_push(b.wrapping_div(a));
                }
            }

            _ => (),
        }
    }
}
impl Processor {
    pub fn new() -> Processor {
        Processor {
            active: false,
            gene_index: 0,
            pc: 0,
            data_stack_index: 0,
            instruction_stack_index: 0,
            data_stack: [0; DATA_STACK_SIZE],
            instruction_stack: [Instr::Noop; INSTRUCTION_STACK_SIZE],
        }
    }

    fn data_push(&mut self, value: i8) {
        // compress stack if needed
        if self.data_stack_index >= DATA_STACK_SIZE {
            self.data_stack_index = DATA_STACK_HALF_SIZE;
            for i in 0..DATA_STACK_HALF_SIZE {
                self.data_stack[i] = self.data_stack[i + DATA_STACK_HALF_SIZE];
            }
        }
        self.data_stack[self.data_stack_index] = value;
        self.data_stack_index += 1;
    }

    fn data_pop(&mut self) -> i8 {
        if self.data_stack_index == 0 {
            0
        } else {
            self.data_stack_index -= 1;
            self.data_stack[self.data_stack_index]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_stack() {
        let mut p = Processor::new();
        p.data_push(1);
        assert_eq!(p.data_stack_index, 1);
        p.data_push(2);
        assert_eq!(p.data_stack_index, 2);
        assert_eq!(p.data_pop(), 2);
        assert_eq!(p.data_pop(), 1);
        // stack underflow
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_data_stack_overflow() {
        let mut p = Processor::new();
        for i in 0..DATA_STACK_SIZE {
            p.data_push(i as i8);
        }
        assert_eq!(p.data_stack_index, DATA_STACK_SIZE);
        // now smash the stack
        p.data_push(100);
        assert_eq!(p.data_stack_index, DATA_STACK_HALF_SIZE + 1);
        assert_eq!(p.data_pop(), 100);
        assert_eq!(p.data_pop(), DATA_STACK_SIZE as i8 - 1)
    }

    #[test]
    fn test_instr_number() {
        let mut p = Processor::new();
        Instr::Number(15).execute(&mut p);
        assert_eq!(p.data_pop(), 15);
    }

    #[test]
    fn test_instr_noop() {
        let mut p = Processor::new();
        Instr::Noop.execute(&mut p);
        let pristine = Processor::new();
        assert_eq!(p, pristine);
    }

    #[test]
    fn test_instr_add() {
        let mut p = Processor::new();
        Instr::Number(2).execute(&mut p);
        Instr::Number(3).execute(&mut p);
        Instr::Add.execute(&mut p);
        assert_eq!(p.data_pop(), 5);
    }

    #[test]
    fn test_instr_add_overflow() {
        let mut p = Processor::new();
        Instr::Number(127).execute(&mut p);
        Instr::Number(1).execute(&mut p);
        Instr::Add.execute(&mut p);
        assert_eq!(p.data_pop(), -128);
    }

    #[test]
    fn test_instr_sub() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Number(3).execute(&mut p);
        Instr::Sub.execute(&mut p);
        assert_eq!(p.data_pop(), 2);
    }

    #[test]
    fn test_instr_sub_underflow() {
        let mut p = Processor::new();
        Instr::Number(-128).execute(&mut p);
        Instr::Number(2).execute(&mut p);
        Instr::Sub.execute(&mut p);
        assert_eq!(p.data_pop(), 126);
    }

    #[test]
    fn test_instr_mul() {
        let mut p = Processor::new();
        Instr::Number(2).execute(&mut p);
        Instr::Number(3).execute(&mut p);
        Instr::Mul.execute(&mut p);
        assert_eq!(p.data_pop(), 6);
    }

    #[test]
    fn test_instr_div() {
        let mut p = Processor::new();
        Instr::Number(6).execute(&mut p);
        Instr::Number(2).execute(&mut p);
        Instr::Div.execute(&mut p);
        assert_eq!(p.data_pop(), 3);
    }

    #[test]
    fn test_instr_div_zero() {
        let mut p = Processor::new();
        Instr::Number(6).execute(&mut p);
        Instr::Number(0).execute(&mut p);
        Instr::Div.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
    }
}
