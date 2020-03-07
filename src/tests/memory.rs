use crate::cpu::WordType;
use crate::memory::{
  Memory as Subject,
  MemoryErr
};

#[test]
fn get() {
  let mut subject = Subject::new(8);
  subject.raw_mut()[0] = 16;

  assert_eq!(Ok(16), subject.get(0));
  assert_eq!(Ok(0), subject.get(1));
}

#[test]
fn get_same() {
  let mut subject = Subject::new(8);
  subject.raw_mut()[0] = 16;

  assert_eq!(Ok(16), subject.get(0));
  assert_eq!(Ok(16), subject.get(0));
}

#[test]
fn get_out_of_range() {
  let size = 8;
  let pos = 9;
  let subject = Subject::new(size);

  assert_eq!(Err(MemoryErr::PointerOutOfRange(size, pos)), subject.get(pos));
}

#[test]
fn get_range() {
  let data: [u16; 4] = [12, 13, 24, 33];
  let mut subject = Subject::new(8);

  subject.raw_mut()[0..4].clone_from_slice(&data);

  assert_eq!(Ok(&data[0..]), subject.get_range(0, 4));
}

#[test]
fn get_range_overflow() {
  let size = 8;
  let pos = 4;
  let count = 5;
  let subject = Subject::new(size);

  assert_eq!(Err(MemoryErr::PointerRangeOverflow(size, pos, count)), subject.get_range(pos, count));
}

#[test]
fn set() {
  let size = 8;
  let pos = 4;
  let value = 1374;

  let mut subject = Subject::new(size);

  assert_eq!(Ok(()), subject.set(pos, value));

  assert_eq!(value, subject.raw()[4]);
}

#[test]
fn set_out_of_range() {
  let size = 8;
  let pos = 9;
  let value = 1374;

  let mut subject = Subject::new(size);

  assert_eq!(Err(MemoryErr::PointerOutOfRange(size, pos)), subject.set(pos, value));
}

#[test]
fn set_range() {
  let size = 4;
  let data: [u16; 4] = [12, 13, 24, 33];
  let mut subject = Subject::new(8);
  assert_eq!(Ok(()), subject.set_range(0, &data));


  assert_eq!(data[0..], subject.raw()[0..size]);
}

#[test]
fn set_range_overflow() {
  let size = 8;
  let pos = 9;
  let data: [u16; 4] = [12, 13, 24, 33];
  let mut subject = Subject::new(size);
  assert_eq!(Err(MemoryErr::PointerRangeOverflow(size, pos, pos + (data.len() as WordType))), subject.set_range(pos, &data));
}