const GENE_SIZE: usize = 32;
const GENE_AMOUNT: usize = 16;
const PROCESSOR_AMOUNT: usize = 4;
const DATA_STACK_SIZE: usize = 32;
const DATA_STACK_HALF_SIZE: usize = DATA_STACK_SIZE / 2;
const INSTRUCTION_STACK_SIZE: usize = 32;
const INSTRUCTION_STACK_HALF_SIZE: usize = INSTRUCTION_STACK_SIZE / 2;

#[derive(Debug, Copy, Clone)]
enum Instr {
    Number(u8),
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

#[derive(Debug, Copy, Clone)]
struct Processor {
    active: bool,
    pc: u8,
    data_stack_index: usize,
    instruction_stack_index: usize,
    data_stack: [u8; DATA_STACK_SIZE],
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
            _ => (),
        }
    }
}
impl Processor {
    pub fn new() -> Processor {
        Processor {
            active: false,
            pc: 0,
            data_stack_index: 0,
            instruction_stack_index: 0,
            data_stack: [0; DATA_STACK_SIZE],
            instruction_stack: [Instr::Noop; INSTRUCTION_STACK_SIZE],
        }
    }

    fn data_push(&mut self, value: u8) {
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

    fn data_pop(&mut self) -> u8 {
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
            p.data_push(i as u8);
        }
        assert_eq!(p.data_stack_index, DATA_STACK_SIZE);
        // now smash the stack
        p.data_push(100);
        assert_eq!(p.data_stack_index, DATA_STACK_HALF_SIZE + 1);
        assert_eq!(p.data_pop(), 100);
        assert_eq!(p.data_pop(), DATA_STACK_SIZE as u8 - 1)
    }
}
