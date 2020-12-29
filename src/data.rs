use std::num::Wrapping;

const GENE_SIZE: usize = 32;
const GENE_AMOUNT: usize = 16;
const PROCESSOR_AMOUNT: usize = 4;
const DATA_STACK_SIZE: usize = 32;
const DATA_STACK_HALF_SIZE: usize = DATA_STACK_SIZE / 2;
const INSTRUCTION_STACK_SIZE: usize = 32;
const INSTRUCTION_STACK_HALF_SIZE: usize = INSTRUCTION_STACK_SIZE / 2;
const CALL_STACK_SIZE: u8 = 32;
const CALL_STACK_HALF_SIZE: u8 = CALL_STACK_SIZE / 2;

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
    Return,

    // Cells where we read and write
    Reading,
    Writing,

    // Conditionally execute the next instruction
    Cond,

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
    gene_index: u8,
    pc: usize,
    cond: bool,
    reading: u8,
    writing: u8,
    data_stack_index: usize,
    call_stack_index: u8,
    instruction_stack_index: usize,
    data_stack: [i8; DATA_STACK_SIZE],
    call_stack: [(u8, usize); CALL_STACK_SIZE as usize],
    instruction_stack: [Instr; INSTRUCTION_STACK_SIZE],
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    genes: [[Instr; GENE_SIZE]; GENE_AMOUNT],
    processors: [Processor; PROCESSOR_AMOUNT],
}

impl Instr {
    fn execute(&self, processor: &mut Processor) {
        if !processor.cond {
            processor.cond = true;
            return;
        }
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
            Instr::Eq => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push((a == b) as i8);
            }
            Instr::Ne => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push((a != b) as i8);
            }
            Instr::Gt => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push((b > a) as i8);
            }
            Instr::Lt => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push((b < a) as i8);
            }
            Instr::And => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push((a != 0 && b != 0) as i8);
            }
            Instr::Or => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push((a != 0 || b != 0) as i8);
            }
            Instr::Not => {
                let a = processor.data_pop();
                processor.data_push((a == 0) as i8);
            }
            Instr::Dup => {
                let a = processor.data_pop();
                processor.data_push(a);
                processor.data_push(a);
            }
            Instr::Drop => {
                processor.data_pop();
            }
            Instr::Swap => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push(a);
                processor.data_push(b);
            }
            Instr::Over => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push(b);
                processor.data_push(a);
                processor.data_push(b);
            }
            Instr::Dup2 => {
                let a = processor.data_pop();
                let b = processor.data_pop();
                processor.data_push(b);
                processor.data_push(a);
                processor.data_push(b);
                processor.data_push(a);
            }
            Instr::Drop2 => {
                processor.data_pop();
                processor.data_pop();
            }
            Instr::Call => {
                let gene_index = processor.data_pop();
                processor.call_push((gene_index as u8) % (GENE_AMOUNT as u8));
            }
            Instr::Return => {
                processor.call_pop();
            }
            Instr::Reading => {
                let a = processor.data_pop();
                processor.reading = (a % 5) as u8; // self plus cardinal directions
            }
            Instr::Writing => {
                let a = processor.data_pop();
                processor.writing = (a % 5) as u8; // self plus cardinal directions
            }
            Instr::Cond => {
                let a = processor.data_pop();
                processor.cond = a != 0;
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
            cond: true,
            reading: 0,
            writing: 0,
            data_stack_index: 0,
            call_stack_index: 0,
            instruction_stack_index: 0,
            data_stack: [0; DATA_STACK_SIZE],
            call_stack: [(0, 0); CALL_STACK_SIZE as usize],
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

    fn call_push(&mut self, gene_index: u8) {
        // compress stack if needed
        if self.call_stack_index >= CALL_STACK_SIZE {
            self.call_stack_index = CALL_STACK_HALF_SIZE;
            for i in 0..CALL_STACK_HALF_SIZE {
                self.call_stack[i as usize] = self.call_stack[(i + CALL_STACK_HALF_SIZE) as usize];
            }
        }
        self.call_stack[self.call_stack_index as usize] = (self.gene_index, self.pc);
        self.call_stack_index += 1;
        self.pc = 0;
        self.gene_index = gene_index;
    }

    fn call_pop(&mut self) {
        let gene_index: u8;
        let pc;
        if self.call_stack_index == 0 {
            gene_index = 0;
            pc = 0;
        } else {
            self.call_stack_index -= 1;
            let top = self.call_stack[self.call_stack_index as usize];
            gene_index = top.0;
            pc = top.1;
        }
        self.gene_index = gene_index;
        self.pc = pc;
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

    #[test]
    fn test_instr_eq_true() {
        let mut p = Processor::new();
        Instr::Number(6).execute(&mut p);
        Instr::Number(6).execute(&mut p);
        Instr::Eq.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }

    #[test]
    fn test_instr_ne_true() {
        let mut p = Processor::new();
        Instr::Number(6).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Ne.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }

    #[test]
    fn test_instr_eq_false() {
        let mut p = Processor::new();
        Instr::Number(6).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Eq.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_gt_true() {
        let mut p = Processor::new();
        Instr::Number(6).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Gt.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }

    #[test]
    fn test_instr_gt_false() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Number(6).execute(&mut p);
        Instr::Gt.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_lt_true() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Number(6).execute(&mut p);
        Instr::Lt.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }

    #[test]
    fn test_instr_and_true() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Number(6).execute(&mut p);
        Instr::And.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }
    #[test]
    fn test_instr_and_false() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Number(0).execute(&mut p);
        Instr::And.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_or_true() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Number(0).execute(&mut p);
        Instr::Or.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }
    #[test]
    fn test_instr_or_false() {
        let mut p = Processor::new();
        Instr::Number(0).execute(&mut p);
        Instr::Number(0).execute(&mut p);
        Instr::Or.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_not() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Not.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
        Instr::Number(0).execute(&mut p);
        Instr::Not.execute(&mut p);
        assert_eq!(p.data_pop(), 1);
    }

    #[test]
    fn test_instr_dup() {
        let mut p = Processor::new();
        Instr::Number(5).execute(&mut p);
        Instr::Dup.execute(&mut p);
        assert_eq!(p.data_pop(), 5);
        assert_eq!(p.data_pop(), 5);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_drop() {
        let mut p = Processor::new();
        Instr::Number(4).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Drop.execute(&mut p);
        assert_eq!(p.data_pop(), 4);
    }

    #[test]
    fn test_instr_swap() {
        let mut p = Processor::new();
        Instr::Number(4).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Swap.execute(&mut p);
        assert_eq!(p.data_pop(), 4);
        assert_eq!(p.data_pop(), 5);
    }

    #[test]
    fn test_instr_over() {
        let mut p = Processor::new();
        Instr::Number(4).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Over.execute(&mut p);
        assert_eq!(p.data_pop(), 4);
        assert_eq!(p.data_pop(), 5);
        assert_eq!(p.data_pop(), 4);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_dup2() {
        let mut p = Processor::new();
        Instr::Number(4).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Dup2.execute(&mut p);
        assert_eq!(p.data_pop(), 5);
        assert_eq!(p.data_pop(), 4);
        assert_eq!(p.data_pop(), 5);
        assert_eq!(p.data_pop(), 4);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_drop2() {
        let mut p = Processor::new();
        Instr::Number(4).execute(&mut p);
        Instr::Number(5).execute(&mut p);
        Instr::Drop2.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
    }

    #[test]
    fn test_instr_call() {
        let mut p = Processor::new();
        Instr::Number(3).execute(&mut p);
        Instr::Call.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
        assert_eq!(p.gene_index, 3);
        assert_eq!(p.call_stack_index, 1);
        assert_eq!(p.call_stack[0], (0, 0));
        Instr::Number(2).execute(&mut p);
        Instr::Call.execute(&mut p);
        assert_eq!(p.gene_index, 2);
        assert_eq!(p.call_stack_index, 2);
        assert_eq!(p.call_stack[1], (3, 0));
    }

    #[test]
    fn test_instr_return() {
        let mut p = Processor::new();
        Instr::Number(3).execute(&mut p);
        Instr::Call.execute(&mut p);
        assert_eq!(p.gene_index, 3);
        assert_eq!(p.call_stack_index, 1);
        assert_eq!(p.call_stack[0], (0, 0));
        Instr::Return.execute(&mut p);
        assert_eq!(p.gene_index, 0);
        assert_eq!(p.call_stack_index, 0);
    }

    #[test]
    fn test_instr_return_underflow() {
        let mut p = Processor::new();
        Instr::Return.execute(&mut p);
        assert_eq!(p.gene_index, 0);
        assert_eq!(p.call_stack_index, 0);
    }

    #[test]
    fn test_instr_call_out_of_range() {
        let mut p = Processor::new();
        // silly non-existing processor number still works
        Instr::Number(-17).execute(&mut p);
        Instr::Call.execute(&mut p);
        assert_eq!(p.data_pop(), 0);
        assert_eq!(p.gene_index, 15);
    }

    #[test]
    fn test_instr_call_stack_overflow() {
        let mut p = Processor::new();
        for i in 1..=CALL_STACK_SIZE {
            Instr::Number(i as i8).execute(&mut p);
            Instr::Call.execute(&mut p);
        }
        assert_eq!(p.call_stack_index, CALL_STACK_SIZE);
        // last invoker
        assert_eq!(
            p.call_stack[CALL_STACK_SIZE as usize - 1],
            (GENE_AMOUNT as u8 - 1, 0)
        );

        // now smash the stack
        Instr::Number(2).execute(&mut p);
        Instr::Call.execute(&mut p);

        assert_eq!(p.call_stack_index, CALL_STACK_HALF_SIZE + 1);
        // 0 as it is just beyond GENE_AMOUNT
        assert_eq!(p.call_stack[p.call_stack_index as usize - 1], (0, 0));
        assert_eq!(p.gene_index, 2);
    }

    #[test]
    fn test_instr_cond_true() {
        let mut p = Processor::new();
        Instr::Number(3).execute(&mut p);
        Instr::Number(4).execute(&mut p);
        Instr::Number(1).execute(&mut p);
        Instr::Cond.execute(&mut p);
        Instr::Add.execute(&mut p);
        assert_eq!(p.data_pop(), 7);
    }

    #[test]
    fn test_instr_cond_false() {
        let mut p = Processor::new();
        Instr::Number(3).execute(&mut p);
        Instr::Number(4).execute(&mut p);
        Instr::Number(0).execute(&mut p);
        Instr::Cond.execute(&mut p);
        Instr::Add.execute(&mut p);
        assert_eq!(p.data_pop(), 4);
    }
}
