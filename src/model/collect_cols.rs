use crate::model::cells::Cells;
use crate::model::col::Col;
use crate::model::cols::Cols;

pub fn collect_cols(cells: &Cells) -> Cols {
    /* 
    for offset in 0..9 {
        let v = cells.clone().values();
        let mut col = Vec::new();
        for (i, c) in v.into_iter().enumerate() {
            if (i + offset) % 9 == 0 {
                col.push(c);
            }
        }
        cols.push(Col::new(col))
    }
    
     */
    Cols::new(Vec::new())
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
        let cols = collect_cols(cells);
    }
}
