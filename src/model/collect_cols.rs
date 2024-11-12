use std::array;
use crate::model::cells::Cells;
use crate::model::col::Col;
use crate::model::cols::Cols;

pub fn collect_cols(cells: &Cells) -> Cols {
    let mut cols = Cols::new();
    let chunks = cells.get_chunks(9);
    for (i,c) in chunks.enumerate() {
        let arr = array::from_fn(|i| c[i]);
        cols.add_col(i,Col::with_cells(arr))
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
        let expected = array::from_fn(|i| Cell::new((i + 1) as u8));
        let expected_col = Col::with_cells(expected);

        let cols = collect_cols(&cells);
        cols.iter().for_each(|c| assert_eq!(c.values(),expected_col.values() ))
    }
}
