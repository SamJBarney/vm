#[cfg(test)]
mod tests;
mod cpu;
mod instruction;
mod memory;

use instruction::Instruction;
use cpu::*;
use memory::*;

fn main() {
    let mut processor = CPU::new(Memory::new(9), 0, 4);
    let instructions: Box<[WordType]> = vec![
        WordType::from(Instruction::JumpRelative(3, 0)),
        0,
        34,
        53,
        WordType::from(Instruction::LoadRelative(-2)),
        WordType::from(Instruction::Move(1, 0)),
        WordType::from(Instruction::LoadRelative(-3)),
        WordType::from(Instruction::Add(0, 1)),
        WordType::from(Instruction::SaveRelative(-7))
    ].into_boxed_slice();


    match processor.borrow_mem().set_range(0, &instructions) {
        Ok(_) => {
            let count = instructions.len();
            for _ in 0..count {
                match processor.step() {
                    Err(err) => {
                        println!("  : {:?}", err);
                        break;
                    }
                    _ => {}
                }
            }
            println!("{:?}", processor.borrow_mem().get(1));
        },
        Err(err) => {
            println!("Couldn't write program into memory: {:?}", err);
        }
    }
}
