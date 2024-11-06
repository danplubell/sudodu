use crate::model::cells::Cells;
use crate::model::cols::Cols;

pub fn collect_cols(cells: &Cells) -> Cols {
    let mut cols = Cols::new();
    for (i, cell) in cells.values().iter().enumerate() {
        let bucket = i % 9;
        cols.add_to_column(bucket, cell);
    }
    cols
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    use crate::model::collect_cols::collect_cols;

    #[test]
    fn test_id() {
        // create vector of buckets
        // go through list and put cells in buckets
        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let s9 = "123456789";
        let ss9 = vec![s9; 9];
        let sol9 = ss9.join("");

        let cells = Cells::from(sol9.as_str());
        let mut cols = collect_cols(&cells);
        let col0 = vec![Cell::new(1);9];
        let col1 = vec![Cell::new(2);9];
        let col2 = vec![Cell::new(3);9];
        let col3 = vec![Cell::new(4);9];
        let col4 = vec![Cell::new(5);9];
        let col5 = vec![Cell::new(6);9];
        let col6 = vec![Cell::new(7);9];
        let col7 = vec![Cell::new(8);9];
        let col8 = vec![Cell::new(9);9];


        assert_eq!(*cols.values()[0].values(), col0);
        assert_eq!(*cols.values()[1].values(), col1);
        assert_eq!(*cols.values()[2].values(), col2);
        assert_eq!(*cols.values()[3].values(), col3);
        assert_eq!(*cols.values()[4].values(), col4);
        assert_eq!(*cols.values()[5].values(), col5);
        assert_eq!(*cols.values()[6].values(), col6);
        assert_eq!(*cols.values()[7].values(), col7);
        assert_eq!(*cols.values()[8].values(), col8);
    }
}
