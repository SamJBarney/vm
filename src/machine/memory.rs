use std::sync::Arc;
use std::sync::RwLock;

use super::word::Type as WordType;
use crate::shared_arc::SharedArc;


pub struct Memory {
  mem: Box<[WordType]>,
}

pub type SharedMemory = SharedArc<Memory>;

impl Memory {
  pub fn new(size: WordType) -> SharedMemory {
    let mem = Self::new_raw(size);
    Arc::new(RwLock::new(mem))
  }

  pub fn new_raw(size: WordType) -> Self {
    Memory {
      mem: vec![0; size as usize].into_boxed_slice()
    }
  }

  pub fn get(&self, pos: WordType) -> Result<WordType, MemoryErr> {
    if pos < self.len() {
      Ok(self.mem[pos as usize])
    } else {
      Err(MemoryErr::PointerOutOfRange(self.len() as WordType, pos))
    }
  }

  pub fn get_range(&self, pos: WordType, count: WordType) -> Result<&[WordType], MemoryErr> {
    if (pos + count) <= self.len() {
      Ok(&self.mem[(pos as usize)..(count as usize)])
    } else {
      Err(MemoryErr::PointerRangeOverflow(self.len() as WordType, pos, count))
    }
  }

  pub fn set (&mut self, pos: WordType, value: WordType) -> Result<(), MemoryErr> {
    if pos < self.len() {
      self.mem[pos as usize] = value;
      Ok(())
    } else {
      Err(MemoryErr::PointerOutOfRange(self.len() as WordType, pos))
    }
  }

  pub fn set_range(&mut self, pos: WordType, range: &[WordType]) -> Result<(), MemoryErr> {
    if (pos as usize + range.len()) <= self.mem.len() {
      self.mem[(pos as usize)..range.len()].clone_from_slice(range);
      Ok(())
    } else {
      Err(MemoryErr::PointerRangeOverflow(self.len() as WordType, pos, pos + (range.len() as WordType)))
    }
  }

  pub fn len(&self) -> WordType {
    self.mem.len() as WordType
  }

  #[cfg(test)]
  pub fn raw(&self) -> &[WordType] {
    &self.mem
  }

  #[cfg(test)]
  pub fn raw_mut(&mut self) -> &mut [WordType] {
    &mut self.mem
  }
}

#[derive(Debug, PartialEq)]
pub enum MemoryErr {
  PointerOutOfRange(WordType, WordType),
  PointerRangeOverflow(WordType, WordType, WordType)
}