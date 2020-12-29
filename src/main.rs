use bevy::prelude::*;

mod data;

// how do we have multiple processors that work on multiple codes?
// i.e. there's a one to many relationship from code to processor
// plus processors can *move* to other codes

// we could make an entity a combination code, processor
// whenever a call or return happens, the old entity is retired
// the new entity is created
// but that could result in a *lot* of entity churn?

// to make this ECS happy we'd need as much data locality as possible, so no vec

// replicator

// enum Instr {
//     // basic numbers
//     Zero,
//     One,

//     // stack manipulation
//     Dup,
//     Swap,
//     Drop,

//     // arithmetic
//     Add,
//     Sub,
//     Mul,
//     Div,

//     // boolean
//     And,
//     Or,
//     Not,

//     // Comparison
//     Gt,
//     Lt,
//     Eq,

//     // Conditional
//     If,

//     // Read/Write instruction data stack
//     Read,  // id, index
//     Write, // id, index, value

//     // Self id
//     Id,

//     // Replication
//     Spawn, // returns id
//     Start, // new processor, id

//     // function calls
//     Call,
//     Ret,
//     // Noop
//     Noop,
// }

// const replicator: [Instr; 32] = [Instr::Id, Instr::Zero, Noop, Noop, Noop];

struct Position {
    x: u64,
    y: u64,
}
// struct Gene {
//     id: i64,
//     code: [Instruction; 32],
// }

// struct Processor {
//     position: u8,
//     data_stack: [i64; 256],
//     instruction_stack: [Instruction; 32],
// }

fn print_position_system(query: Query<&Position>) {
    for position in query.iter() {
        println!("position: {:?} {:?}", position.x, position.y);
    }
}

fn hello_world() {
    println!("hello world!");
}

#[bevy_main]
fn main() {
    App::build().add_system(hello_world.system()).run();
}

// https://gamedev.stackexchange.com/questions/154206/are-references-between-entities-in-entitiy-component-system-allowed
// I think the processor component would need some kind of reference to
// a gene component, but how can that work?
