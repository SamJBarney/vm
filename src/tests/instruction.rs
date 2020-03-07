use crate::instruction::Instruction as Subject;
use crate::cpu::WordType;
#[test]
fn from_word_type() {
  assert_eq!(Subject::Nop, Subject::from(0b0000000000000000));
  assert_eq!(Subject::PushRegister(1), Subject::from(0b0000000000100001));
  assert_eq!(Subject::PopRegister(1), Subject::from(0b0000000000100010));
  assert_eq!(Subject::PushRegisters, Subject::from(0b0000000000000011));
  assert_eq!(Subject::PopRegisters, Subject::from(0b0000000000000100));
  assert_eq!(Subject::Move(1, 1), Subject::from(0b0000001000100101));
  assert_eq!(Subject::Load(1, 1), Subject::from(0b0000001000100110));
  assert_eq!(Subject::Save(1, 1), Subject::from(0b0000001000100111));
  assert_eq!(Subject::Add(1, 1), Subject::from(0b0000001000101000));
  assert_eq!(Subject::Subtract(1, 1), Subject::from(0b0000001000101001));
  assert_eq!(Subject::Multiply(1, 1), Subject::from(0b0000001000101010));
  assert_eq!(Subject::Divide(1, 1), Subject::from(0b0000001000101011));
  assert_eq!(Subject::Equal(1, 1), Subject::from(0b0000001000101100));
  assert_eq!(Subject::NotEqual(1, 1), Subject::from(0b0000001000101101));
  assert_eq!(Subject::GreaterThan(1, 1), Subject::from(0b0000001000101110));
  assert_eq!(Subject::LessThan(1, 1), Subject::from(0b0000001000101111));
  assert_eq!(Subject::Xor(1, 1), Subject::from(0b0000001000110000));
  assert_eq!(Subject::Not(1), Subject::from(0b0000000000110001));
  assert_eq!(Subject::Jump(1, 0), Subject::from(0b0000000000110010));
  assert_eq!(Subject::Interrupt(1), Subject::from(0b0000000100110011));
  assert_eq!(Subject::BitShiftLeft(1, 1), Subject::from(0b0000001000110101));
  assert_eq!(Subject::BitShiftRight(1, 1), Subject::from(0b0000001000110110));
  assert_eq!(Subject::BitNot(1), Subject::from(0b0000000000110111));
  assert_eq!(Subject::BitXor(1, 1), Subject::from(0b0000001000111000));
  assert_eq!(Subject::BitAnd(1, 1), Subject::from(0b0000001000111001));
  assert_eq!(Subject::BitOr(1, 1), Subject::from(0b0000001000111010));
  assert_eq!(Subject::BitNor(1, 1), Subject::from(0b0000001000111011));
  assert_eq!(Subject::LoadRelative(-2), Subject::from(0b1111111000111100));
  assert_eq!(Subject::JumpRelative(-2, 1), Subject::from(0b1111111000111101));
}

#[test]
fn from_instruction() {
  assert_eq!(0b0000000000000000, WordType::from(Subject::Nop));
  assert_eq!(0b0000000000100001, WordType::from(Subject::PushRegister(1)));
  assert_eq!(0b0000000000100010, WordType::from(Subject::PopRegister(1)));
  assert_eq!(0b0000000000000011, WordType::from(Subject::PushRegisters));
  assert_eq!(0b0000000000000100, WordType::from(Subject::PopRegisters));
  assert_eq!(0b0000001000100101, WordType::from(Subject::Move(1, 1)));
  assert_eq!(0b0000001000100110, WordType::from(Subject::Load(1, 1)));
  assert_eq!(0b0000001000100111, WordType::from(Subject::Save(1, 1)));
  assert_eq!(0b0000001000101000, WordType::from(Subject::Add(1, 1)));
  assert_eq!(0b0000001000101001, WordType::from(Subject::Subtract(1, 1)));
  assert_eq!(0b0000001000101010, WordType::from(Subject::Multiply(1, 1)));
  assert_eq!(0b0000001000101011, WordType::from(Subject::Divide(1, 1)));
  assert_eq!(0b0000001000101100, WordType::from(Subject::Equal(1, 1)));
  assert_eq!(0b0000001000101101, WordType::from(Subject::NotEqual(1, 1)));
  assert_eq!(0b0000001000101110, WordType::from(Subject::GreaterThan(1, 1)));
  assert_eq!(0b0000001000101111, WordType::from(Subject::LessThan(1, 1)));
  assert_eq!(0b0000001000110000, WordType::from(Subject::Xor(1, 1)));
  assert_eq!(0b0000000000110001, WordType::from(Subject::Not(1)));
  assert_eq!(0b0000000000110010, WordType::from(Subject::Jump(1, 0)));
  assert_eq!(0b0000111100010011, WordType::from(Subject::Interrupt(15)));
  assert_eq!(0b0000001000110101, WordType::from(Subject::BitShiftLeft(1, 1)));
  assert_eq!(0b0000001000110110, WordType::from(Subject::BitShiftRight(1, 1)));
  assert_eq!(0b0000000000110111, WordType::from(Subject::BitNot(1)));
  assert_eq!(0b0000001000111000, WordType::from(Subject::BitXor(1, 1)));
  assert_eq!(0b0000001000111001, WordType::from(Subject::BitAnd(1, 1)));
  assert_eq!(0b0000001000111010, WordType::from(Subject::BitOr(1, 1)));
  assert_eq!(0b0000001000111011, WordType::from(Subject::BitNor(1, 1)));
  assert_eq!(0b1111111000011100, WordType::from(Subject::LoadRelative(-2)));
  assert_eq!(0b1111111000111101, WordType::from(Subject::JumpRelative(-2, 1)));
  assert_eq!(0b0000000100011110, WordType::from(Subject::SaveRelative(1)));
}