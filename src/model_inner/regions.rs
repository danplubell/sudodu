use crate::model_inner::cell::Cell;
use crate::model_inner::cells::Cells;
use crate::model_inner::is_safe::is_safe;
use crate::model_inner::validate_cells::validate_cells;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub struct Regions {
    values: Rc<RefCell<Vec<Cells>>>,
}
impl Regions {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new((0..9).map(|_| Cells::new()).collect())),
        }
    }
    pub fn add_to_region(&self, region: usize, cell: Cell) {
        if let Some(reg) = self.values.borrow_mut().get_mut(region) {
            reg.add_cell(cell);
        }
    }
    pub fn get_region(&self, region: usize) -> Cells {
        self.values.borrow().get(region).unwrap().clone()
    }
    pub fn clear_note(&self, region: usize, value: u8) {
        self.values.borrow_mut().get(region).unwrap().clear_note(value);
    }
    pub fn collect_regions(&self, cells: &Cells) {
        // These are the root nodes
        // From each root we iterate from the root to
        let roots: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];
        for (region_idx, root) in roots.iter().enumerate() {
            let start = *root;
            let end = root + 18;
            // move to next frame
            for i in (start..=end).step_by(9) {
                let start = i;
                let end = i + 3;
                // collect cells and add to region
                for j in start..end {
                    self.add_to_region(region_idx, cells.get_at(j));
                }
            }
        }
    }
    pub fn is_valid(&self) -> bool {
        self.values
            .borrow()
            .iter()
            .all(|c| validate_cells(c).is_ok())
    }
    pub fn is_safe(&self) -> bool {
        self.values.borrow().iter().all(|c| is_safe(c).is_ok())
    }
}
#[cfg(test)]
mod tests {
    use crate::model_inner::cell::Cell;
    use crate::model_inner::cells::Cells;
    use crate::model_inner::regions::Regions;

    #[test]
    pub fn test_collect_regions() {
        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let cells = Cells::from(solution);
        let regions = Regions::new();
        let first_region_values = [3u8, 1, 8, 5, 7, 2, 9, 4, 6];
        let last_region_values = [6u8, 8, 9, 3, 5, 4, 2, 1, 7];
        let first_region_cells = Cells::new();
        let last_region_cells = Cells::new();
        first_region_values
            .iter()
            .for_each(|v| first_region_cells.add_cell(Cell::new(*v)));
        last_region_values
            .iter()
            .for_each(|v| last_region_cells.add_cell(Cell::new(*v)));

        regions.collect_regions(&cells);
        let first_region = regions.get_region(0);
        assert_eq!(first_region, first_region_cells);
        let last_region = regions.get_region(8);
        assert_eq!(last_region, last_region_cells);
    }
    #[test]
    pub fn test_clear_note() {
        let regions = Regions::new();
        let cell1 = Cell::new(3);
        cell1.add_note_value(1);
        cell1.add_note_value(2);
        let cell2 = Cell::new(4);
        cell2.add_note_value(1);
        cell2.add_note_value(3);
        regions.add_to_region(0,cell1);
        regions.add_to_region(0, cell2);
        regions.clear_note(0,1);
        println!("{:?}", regions);
    }
}
