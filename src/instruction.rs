use crate::cpu::WordType;

const INSTRUCTION_SIZE: usize = 5;
const INSTRUCTION_MASK: WordType = 0x1F;
const REGISTER_OFFSET: usize = 5;
const REGISTER_SIZE: usize = 4;
const REGISTER_MASK: WordType = 0x0F;

const JUMP_FLAG_OFFSET: usize = 5;
const JUMP_FLAG_MASK: WordType = 0x00E0;

const ARG_OFFSET: usize = 8;
const ARG_MASK: WordType = 0xFF00;

macro_rules! get_instruction {
  ($value:ident) => {
    ($value & INSTRUCTION_MASK)
  }
}

macro_rules! get_register {
  ($value:ident, $reg:expr) => {
    (($value & (REGISTER_MASK << (REGISTER_SIZE * $reg + REGISTER_OFFSET)) as WordType) >> (REGISTER_SIZE * $reg + REGISTER_OFFSET)) as u8
  };
}

macro_rules! get_unsigned_arg {
  ($value:ident) => {
    ((($value & ARG_MASK) >> ARG_OFFSET) as u8)
  };
}

macro_rules! get_relative {
  ($value:ident) => {
    ((get_unsigned_arg!($value)) as i8)
  };
}

macro_rules! get_jump_flags {
  ($value:ident) => {
    (($value & JUMP_FLAG_MASK) >> JUMP_FLAG_OFFSET) as u8
  };
}

macro_rules! set_register {
  ($value:ident, $reg:expr, $reg_value:expr) => {
    ($value | (($reg_value as WordType) << (REGISTER_SIZE * $reg + REGISTER_OFFSET)))
  };
  ($value:expr, $reg:expr, $reg_value:expr) => {
    ($value | (($reg_value as WordType) << (REGISTER_SIZE * $reg + REGISTER_OFFSET)))
  };
}

macro_rules! set_unsigned_arg {
  ($value:expr, $rel:expr) => {
    ($value | ((($rel as WordType) << ARG_OFFSET) & ARG_MASK))
  };
}

macro_rules! set_relative {
  ($value:expr, $rel:expr) => {
    set_unsigned_arg!($value, $rel)
  };
}

macro_rules! set_jump_flags {
  ($value:expr, $rel:expr) => {
    ($value | ((($rel as WordType) << JUMP_FLAG_OFFSET) & JUMP_FLAG_MASK))
  };
}

macro_rules! inst {
  ($name:ident, $value:expr) => {
    pub const $name: WordType = $value;
  };
}

pub mod codes {
  use crate::cpu::WordType;

  inst!(NOP, 0);
  inst!(PUSH, 1);
  inst!(POP, 2);
  inst!(PUSHS, 3);
  inst!(POPS, 4);
  inst!(MOVE_RR, 5);
  inst!(LD, 6);
  inst!(SAV, 7);
  inst!(ADD, 8);
  inst!(SUB, 9);
  inst!(MUL, 10);
  inst!(DIV, 11);
  inst!(CMP_EQ, 12);
  inst!(CMP_NE, 13);
  inst!(CMP_GT, 14);
  inst!(CMP_LT, 15);
  inst!(CMP_XOR, 16);
  inst!(CMP_NOT, 17);
  inst!(JMP, 18);
  inst!(INT, 19);
  inst!(UNUSED_2, 20);
  inst!(BSL, 21);
  inst!(BSR, 22);
  inst!(BNOT, 23);
  inst!(BXOR, 24);
  inst!(BAND, 25);
  inst!(BOR, 26);
  inst!(BNOR, 27);
  inst!(LD_REL, 28);
  inst!(JREL, 29);
  inst!(SAV_REL, 30);
  inst!(UNUSED_4, 31);
}

use codes::*;

#[derive(Debug, PartialEq)]
pub enum Instruction {
  Nop,
  PushRegister(u8),
  PopRegister(u8),
  PushRegisters,
  PopRegisters,
  Move(u8, u8),
  Load(u8, u8),
  Save(u8, u8),
  Add(u8, u8),
  Subtract(u8, u8),
  Multiply(u8, u8),
  Divide(u8, u8),
  Equal(u8, u8),
  NotEqual(u8, u8),
  GreaterThan(u8, u8),
  LessThan(u8, u8),
  Xor(u8, u8),
  Not(u8),
  Jump(u8, u8),
  Interrupt(u8),
  BitShiftLeft(u8, u8),
  BitShiftRight(u8, u8),
  BitNot(u8),
  BitXor(u8, u8),
  BitAnd(u8, u8),
  BitOr(u8, u8),
  BitNor(u8, u8),
  LoadRelative(i8),
  SaveRelative(i8),
  JumpRelative(i8, u8),
  Invalid(WordType)
}

impl From<&WordType> for Instruction {
  fn from(value: &WordType) -> Instruction {
    Instruction::from(*value)
  }
}

impl From<WordType> for Instruction {
  fn from(value: WordType) -> Instruction {
    match get_instruction!(value) {
      NOP => Instruction::Nop,
      PUSH => Instruction::PushRegister(get_register!(value, 0)),
      POP => Instruction::PopRegister(get_register!(value, 0)),
      PUSHS => Instruction::PushRegisters,
      POPS => Instruction::PopRegisters,
      MOVE_RR => Instruction::Move(get_register!(value, 0), get_register!(value, 1)),
      LD => Instruction::Load(get_register!(value, 0), get_register!(value, 1)),
      SAV => Instruction::Save(get_register!(value, 0), get_register!(value, 1)),
      ADD => Instruction::Add(get_register!(value, 0), get_register!(value, 1)),
      SUB => Instruction::Subtract(get_register!(value, 0), get_register!(value, 1)),
      MUL => Instruction::Multiply(get_register!(value, 0), get_register!(value, 1)),
      DIV => Instruction::Divide(get_register!(value, 0), get_register!(value, 1)),
      CMP_EQ => Instruction::Equal(get_register!(value, 0), get_register!(value, 1)),
      CMP_NE => Instruction::NotEqual(get_register!(value, 0), get_register!(value, 1)),
      CMP_GT => Instruction::GreaterThan(get_register!(value, 0), get_register!(value, 1)),
      CMP_LT => Instruction::LessThan(get_register!(value, 0), get_register!(value, 1)),
      CMP_XOR => Instruction::Xor(get_register!(value, 0), get_register!(value, 1)),
      CMP_NOT => Instruction::Not(get_register!(value, 0)),
      JMP => Instruction::Jump(get_register!(value, 0), get_register!(value, 1)),
      INT => Instruction::Interrupt(get_unsigned_arg!(value)),
      BSL => Instruction::BitShiftLeft(get_register!(value, 0), get_register!(value, 1)),
      BSR => Instruction::BitShiftRight(get_register!(value, 0), get_register!(value, 1)),
      BNOT => Instruction::BitNot(get_register!(value, 0)),
      BXOR => Instruction::BitXor(get_register!(value, 0), get_register!(value, 1)),
      BAND => Instruction::BitAnd(get_register!(value, 0), get_register!(value, 1)),
      BOR => Instruction::BitOr(get_register!(value, 0), get_register!(value, 1)),
      BNOR => Instruction::BitNor(get_register!(value, 0), get_register!(value, 1)),
      LD_REL => Instruction::LoadRelative(get_relative!(value)),
      JREL => Instruction::JumpRelative(get_relative!(value), get_jump_flags!(value)),
      SAV_REL => Instruction::SaveRelative(get_relative!(value)),
      _ => Instruction::Invalid(value)
    }
  }
}

impl From<Instruction> for WordType {
  fn from(value: Instruction) -> WordType {
    WordType::from(&value)
  }
}

impl From<&Instruction> for WordType {
  fn from(value: &Instruction) -> WordType {
    match value {
      Instruction::Nop => NOP,
      Instruction::PushRegister(reg) => set_register!(PUSH, 0, *reg),
      Instruction::PopRegister(reg) => set_register!(POP, 0, *reg),
      Instruction::PushRegisters => PUSHS,
      Instruction::PopRegisters => POPS,
      Instruction::Move(into, from) => set_register!(
        set_register!(MOVE_RR, 0, *into),
        1,
        *from
      ),
      Instruction::Load(into, from) => set_register!(
        set_register!(LD, 0, *into),
        1,
        *from
      ),
      Instruction::Save(into, from) => set_register!(
        set_register!(SAV, 0, *into),
        1,
        *from
      ),
      Instruction::Add(into, from) => set_register!(
        set_register!(ADD, 0, *into),
        1,
        *from
      ),
      Instruction::Subtract(into, from) => set_register!(
        set_register!(SUB, 0, *into),
        1,
        *from
      ),
      Instruction::Multiply(into, from) => set_register!(
        set_register!(MUL, 0, *into),
        1,
        *from
      ),
      Instruction::Divide(into, from) => set_register!(
        set_register!(DIV, 0, *into),
        1,
        *from
      ),
      Instruction::Equal(into, from) => set_register!(
        set_register!(CMP_EQ, 0, *into),
        1,
        *from
      ),
      Instruction::NotEqual(into, from) => set_register!(
        set_register!(CMP_NE, 0, *into),
        1,
        *from
      ),
      Instruction::GreaterThan(left, right) => set_register!(
        set_register!(CMP_GT, 0, *left),
        1,
        *right
      ),
      Instruction::LessThan(left, right) => set_register!(
        set_register!(CMP_LT, 0, *left),
        1,
        *right
      ),
      Instruction::Xor(left, right) => set_register!(
        set_register!(CMP_XOR, 0, *left),
        1,
        *right
      ),
      Instruction::Not(reg) => set_register!(CMP_NOT, 0, *reg),
      Instruction::Jump(reg, flags) => set_register!(
        set_register!(JMP, 0, *reg),
        1,
        *flags
      ),
      Instruction::Interrupt(value) => set_unsigned_arg!(INT, *value),
      Instruction::BitShiftLeft(left, right) => set_register!(
        set_register!(BSL, 0, *left),
        1,
        *right
      ),
      Instruction::BitShiftRight(left, right) => set_register!(
        set_register!(BSR, 0, *left),
        1,
        *right
      ),
      Instruction::BitNot(reg) => set_register!(BNOT, 0, *reg),
      Instruction::BitXor(left, right) => set_register!(
        set_register!(BXOR, 0, *left),
        1,
        *right
      ),
      Instruction::BitAnd(left, right) => set_register!(
        set_register!(BAND, 0, *left),
        1,
        *right
      ),
      Instruction::BitOr(left, right) => set_register!(
        set_register!(BOR, 0, *left),
        1,
        *right
      ),
      Instruction::BitNor(left, right) => set_register!(
        set_register!(BNOR, 0, *left),
        1,
        *right
      ),
      Instruction::LoadRelative(rel) => set_relative!(LD_REL, *rel),
      Instruction::JumpRelative(rel, flags) => set_jump_flags!(set_relative!(JREL, *rel), *flags),
      Instruction::SaveRelative(rel) => set_relative!(SAV_REL, *rel),
      Instruction::Invalid(_) => NOP
    }
  }
}
