use crate::model::cells::Cells;
use crate::model::cols::Cols;

pub fn collect_cols(cells: &Cells) -> Cols {
    let mut cols = Cols::new();
    let chunks = cells.get_chunks(9);
    for (row,c) in chunks.enumerate() {
        for (col, cell) in c.iter().enumerate() {
            cols.add_to_row_column(row, col, *cell );
        }       
    }
    cols
}

#[cfg(test)]
mod tests {
    use std::array;
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    use crate::model::col::Col;
    use crate::model::collect_cols::collect_cols;

    #[test]
    fn test_collect_cols() {
        // create vector of buckets
        // go through list and put cells in buckets
        let s9 = "123456789";
        let ss9 = [s9; 9];
        let sol9 = ss9.join("");

        let cells = Cells::from(sol9.as_str());

        let cols = collect_cols(&cells);
        println!("{:?}", cols);
        for (i,c) in cols.iter().enumerate() {
            let expected = array::from_fn(|_i| Cell::new(i as u8 + 1));
            let expected_col = Col::with_cells(expected);
            assert_eq!(c.values(), expected_col.values());
        }
    }
}
