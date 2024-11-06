use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::col::Col;
use crate::model::cols::Cols;

pub fn collect_cols(cells: &Cells) -> Cols {
    let mut cols = Cols::new();
    for (i,cell) in cells.values().iter().enumerate() {
        let bucket = i%9;
        cols.add_to_column(bucket,cell);
    }
    cols
}

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;
    use std::ops::Index;
    use crate::model::collect_cols::collect_cols;
    //    fn test_collect_cols() {
    //        let solution =
    //            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";

    //        let c = Cells::from(solution);
    //    }
    #[test]
    fn test_id() {
        // create vector of buckets
        // go through list and put cells in buckets
        let solution=
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let cells = Cells::from(solution);
        let cols = collect_cols(&cells);
    }
}
