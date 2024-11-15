use crate::model::cell::Cell;
use crate::model::cells::Cells;

pub struct Column {
    values: Cells,
}
impl Column {
    pub fn new()-> Self{
        Self {
            values: Cells::new()
        }
    }
    pub fn add_cell(&self, cell:Cell) {
        todo!();
    }
}

#[cfg(test)] 
mod tests {
    
}