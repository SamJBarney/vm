use crate::shared_arc::SharedArc;
use std::ops::Deref;

use super::memory::{
  SharedMemory,
  Memory,
  MemoryErr,
};

use super::word;

type WordType = word::Type;

const FLAGS: usize = 13;
const PC: usize = 14;
const STACK_POINTER: usize = 15;

const FLAG_OVERFLOW: WordType = 0x0001;
const FLAG_COMPARISON: WordType = 0x0002;


#[derive(Debug)]
pub enum ProcessorError {
  MemoryErr(MemoryErr),
  StackOverflow,
  StackUnderflow,
  InvalidJumpCondition(u8),
  Unreachable(String)
}

struct Processor {
  registers: [WordType; 16],
  mem: SharedMemory,
  stack: Memory,
  running: bool,
}

impl Processor {
  pub fn new(mem: SharedMemory, stack_size: WordType, program_counter: WordType) -> Self {
    let mut processor = Processor {
      registers: [0; 16],
      mem,
      stack: Memory::new_raw(stack_size),
      running: false,
    };
    processor.registers[PC] = program_counter;
    processor
  }

  pub fn step(&mut self) -> Result<(), ProcessorError> {
    let op = self.mem.deref().read().unwrap().get(self.registers[PC]);
    let res = match op {
      Ok(instruction) => self.do_instruction(instruction),
      Err(err) => {
        match err {
          any => {
            self.registers[PC] = word::MAX;
            Err(ProcessorError::MemoryErr(any))
          }
        }
      }
    };

    self.registers[PC] = self.registers[PC].wrapping_add(1) % self.mem.deref().read().unwrap().len();
    res
  }

  fn do_instruction(&mut self, instruction: WordType) -> Result<(), ProcessorError> {
    Ok(())
    // match Instruction::from(instruction) {
    //   Instruction::PushRegister(reg) => {
    //     // Attempt to push the value onto the stack
    //     match self.stack.set(self.registers[STACK_POINTER], self.registers[reg as usize]) {
    //       // Valid stack position
    //       Ok(()) => {

    //         // Increment the stack pointer
    //         self.registers[STACK_POINTER] += 1;
    //         Ok(())
    //       },

    //       // Handle a stack overflow
    //       Err(_) => Err(ProcessorError::StackOverflow)
    //     }
    //   },
    //   Instruction::PopRegister(reg) => {
    //     match self.registers[STACK_POINTER].checked_sub(1) {
    //       // Valid stack position
    //       Some(pos) => {
    //         // Save stack position
    //         self.registers[STACK_POINTER] = pos;

    //         // Retrieve the stack value and put it on the register
    //         match self.stack.get(pos) {
    //           Ok(value) => {
    //             self.registers[reg as usize] = value;
    //             Ok(())
    //           },
    //           _ => Err(ProcessorError::Unreachable(String::from("This should never happen because we are already checking that we are within the stack boundaries")))
    //         }
    //       },

    //       // Handle a stack underflow
    //       None => Err(ProcessorError::StackUnderflow)
    //     }
    //   },
    //   Instruction::PushRegisters => {
    //     match self.stack.set_range(self.registers[STACK_POINTER], &self.registers[0..FLAGS]) {
    //       Ok(()) => {
    //         self.registers[STACK_POINTER] += FLAGS as WordType;
    //         Ok(())
    //       },
    //       _ => Err(ProcessorError::StackOverflow)
    //     }
    //   },
    //   Instruction::PopRegisters => {
    //     match self.registers[STACK_POINTER].checked_sub(FLAGS as WordType) {
    //       Some(pos) => {
    //         self.registers[STACK_POINTER] = pos;
    //         match self.stack.get_range(pos, FLAGS as WordType) {
    //           Ok(regs) => {
    //             &self.registers[0..FLAGS].clone_from_slice(regs);
    //             Ok(())
    //           },
    //           _ => Err(ProcessorError::Unreachable(String::from("This should never happen because we are already checking that we are within the stack boundaries")))
    //         }
    //       },
    //       _ => Err(ProcessorError::StackUnderflow)
    //     }
    //   },
    //   Instruction::Move(into, from) => {
    //     self.registers[into as usize] = self.registers[from as usize];
    //     Ok(())
    //   },
    //   Instruction::Load(into, src) => {
    //     let pointer: WordType = self.registers[src as usize];
    //     match self.memory.get(pointer) {
    //       Ok(value) => {
    //         self.registers[into as usize] = value;
    //         Ok(())
    //       },
    //       Err(err) => Err(ProcessorError::MemoryErr(err))
    //     }
    //   },
    //   Instruction::Save(into, from) => {
    //     match self.memory.set(self.registers[into as usize], self.registers[from as usize]) {
    //       Ok(()) => Ok(()),
    //       Err(err) => Err(ProcessorError::MemoryErr(err))
    //     }
    //   },
    //   Instruction::Add(into, from) => {
    //     let val1 = self.registers[into as usize];
    //     let val2 = self.registers[from as usize];
    //     let (result, overflow) = val1.overflowing_add(val2);
    //     self.registers[into as usize] = result;

    //     // Update the OVERFLOW flag
    //     if overflow {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_OVERFLOW;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_OVERFLOW;
    //     }
    //     Ok(())
    //   },
    //   Instruction::Subtract(into, from) => {
    //     let val1 = self.registers[into as usize];
    //     let val2 = self.registers[from as usize];
    //     let (result, overflow) = val1.overflowing_sub(val2);
    //     self.registers[into as usize] = result;

    //     // Update the OVERFLOW flag
    //     if overflow {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_OVERFLOW;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_OVERFLOW;
    //     }
    //     Ok(())
    //   },
    //   Instruction::Multiply(into, from) => {
    //     let val1 = self.registers[into as usize];
    //     let val2 = self.registers[from as usize];
    //     let (result, overflow) = val1.overflowing_mul(val2);
    //     self.registers[into as usize] = result;

    //     // Update the OVERFLOW flag
    //     if overflow {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_OVERFLOW;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_OVERFLOW;
    //     }
    //     Ok(())
    //   },
    //   Instruction::Divide(into, from) => {
    //     let val1 = self.registers[into as usize];
    //     let val2 = self.registers[from as usize];
    //     let result = val1 / val2;
    //     self.registers[into as usize] = result;

    //     self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_OVERFLOW;
    //     Ok(())
    //   },
    //   Instruction::Equal(reg1, reg2) => {
    //     if self.registers[reg1 as usize] == self.registers[reg2 as usize] {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_COMPARISON;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_COMPARISON;
    //     }
    //     Ok(())
    //   },
    //   Instruction::NotEqual(reg1, reg2) => {
    //     if self.registers[reg1 as usize] != self.registers[reg2 as usize] {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_COMPARISON;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_COMPARISON;
    //     }
    //     Ok(())
    //   },
    //   Instruction::GreaterThan(reg1, reg2) => {
    //     if self.registers[reg1 as usize] > self.registers[reg2 as usize] {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_COMPARISON;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_COMPARISON;
    //     }
    //     Ok(())
    //   },
    //   Instruction::LessThan(reg1, reg2) => {
    //     if self.registers[reg1 as usize] < self.registers[reg2 as usize] {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_COMPARISON;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_COMPARISON;
    //     }
    //     Ok(())
    //   },
    //   Instruction::Xor(reg1, reg2) => {
    //     let value1 = self.registers[reg1 as usize];
    //     let value2 = self.registers[reg2 as usize];
    //     if (value1 > 0 && value2 > 0) || (value1 == 0 && value2 == 0) {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_COMPARISON;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_COMPARISON;
    //     }
    //     Ok(())
    //   },
    //   Instruction::Not(reg1) => {
    //     if self.registers[reg1 as usize] == 0 {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_COMPARISON;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_COMPARISON;
    //     }
    //     Ok(())
    //   },
    //   Instruction::Jump(reg, condition) => {
    //     match condition {
    //       0 => {
    //         self.registers[PC] = self.registers[reg as usize] - 1;
    //         Ok(())
    //       },
    //       1 => {
    //         if (self.registers[FLAGS] & FLAG_COMPARISON) == FLAG_COMPARISON {
    //           self.registers[PC] = self.registers[reg as usize] - 1;
    //         }
    //         Ok(())
    //       },
    //       2 => {
    //         if (self.registers[FLAGS] & FLAG_COMPARISON) != FLAG_COMPARISON {
    //           self.registers[PC] = self.registers[reg as usize] - 1;
    //         }
    //         Ok(())
    //       },
    //       3 => {
    //         if (self.registers[FLAGS] & FLAG_OVERFLOW) == FLAG_OVERFLOW {
    //           self.registers[PC] = self.registers[reg as usize] - 1;
    //         }
    //         Ok(())
    //       }
    //       any => Err(ProcessorError::InvalidJumpCondition(any))
    //     }
    //   },
    //   Instruction::BitShiftLeft(reg1, reg2) => {
    //     let (result, overflow) = self.registers[reg1 as usize].overflowing_shl(self.registers[reg2 as usize] as u32);
    //     self.registers[reg1 as usize] = result;

    //     // Update the OVERFLOW flag
    //     if overflow {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_OVERFLOW;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_OVERFLOW;
    //     }
    //     Ok(())
    //   },
    //   Instruction::BitShiftRight(reg1, reg2) => {
    //     let (result, overflow) = self.registers[reg1 as usize].overflowing_shr(self.registers[reg2 as usize] as u32);
    //     self.registers[reg1 as usize] = result;

    //     // Update the OVERFLOW flag
    //     if overflow {
    //       self.registers[FLAGS] = self.registers[FLAGS] | FLAG_OVERFLOW;
    //     } else {
    //       self.registers[FLAGS] = self.registers[FLAGS] & !FLAG_OVERFLOW;
    //     }
    //     Ok(())
    //   },
    //   Instruction::BitNot(reg) => {
    //     self.registers[reg as usize] = !self.registers[reg as usize];
    //     Ok(())
    //   },
    //   Instruction::BitXor(reg1, reg2) => {
    //     let result = self.registers[reg1 as usize] ^ self.registers[reg2 as usize];
    //     self.registers[reg1 as usize] = result;

    //     Ok(())
    //   },
    //   Instruction::BitAnd(reg1, reg2) => {
    //     let result = self.registers[reg1 as usize] & self.registers[reg2 as usize];
    //     self.registers[reg1 as usize] = result;

    //     Ok(())
    //   },
    //   Instruction::BitOr(reg1, reg2) => {
    //     let result = self.registers[reg1 as usize] | self.registers[reg2 as usize];
    //     self.registers[reg1 as usize] = result;

    //     Ok(())
    //   },
    //   Instruction::BitNor(reg1, reg2) => {
    //     let result = self.registers[reg1 as usize] | self.registers[reg2 as usize];
    //     self.registers[reg1 as usize] = !result;

    //     Ok(())
    //   },
    //   Instruction::LoadRelative(offset) => {
    //     let position: WordType = ((self.registers[PC] as word::ConversionType) + (offset as word::ConversionType)) as WordType;
    //     match self.memory.get(position) {
    //       Ok(value) => {
    //         self.registers[0] = value;
    //         Ok(())
    //       },
    //       Err(err) => Err(ProcessorError::MemoryErr(err))
    //     }
    //   },
    //   Instruction::SaveRelative(offset) => {
    //     let position: WordType = ((self.registers[PC] as word::ConversionType) + (offset as word::ConversionType)) as WordType;
    //     match self.memory.set(position, self.registers[0]) {
    //       Ok(()) => Ok(()),
    //       Err(err) => Err(ProcessorError::MemoryErr(err))
    //     }
    //   },
    //   Instruction::JumpRelative(offset, condition) => {
    //     let position: WordType = ((self.registers[PC] as word::ConversionType) + (offset as word::ConversionType)) as WordType;
    //     match condition {
    //       0 => {
    //         self.registers[PC] = position;
    //         Ok(())
    //       },
    //       1 => {
    //         if (self.registers[FLAGS] & FLAG_COMPARISON) == FLAG_COMPARISON {
    //           self.registers[PC] = position;
    //         }
    //         Ok(())
    //       },
    //       2 => {
    //         if (self.registers[FLAGS] & FLAG_COMPARISON) != FLAG_COMPARISON {
    //           self.registers[PC] = position;
    //         }
    //         Ok(())
    //       },
    //       3 => {
    //         if (self.registers[FLAGS] & FLAG_OVERFLOW) == FLAG_OVERFLOW {
    //           self.registers[PC] = position;
    //         }
    //         Ok(())
    //       }
    //       any => Err(ProcessorError::InvalidJumpCondition(any))
    //     }
    //   },
    //   _ => Ok(())
    // }
  }
}

pub struct ProcessorActor {
  processor: SharedArc<Processor>,
}