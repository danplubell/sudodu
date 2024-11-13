use crate::model::cells::Cells;
use crate::model::columns::Columns;

pub fn collect_columns<'a>(cells: Cells) -> Columns<'a> {
    let mut cols = Columns::new();
    let chunks = cells.get_chunks(9);
    for (row,c) in chunks.enumerate() {
        for (col, cell) in c.iter().enumerate() {
            cols.add_to_column(col, cell );
        }       
    }
    cols
}

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;
    use crate::model::collect_columns::collect_columns;

    #[test]
    fn test_collect_cols() {
        // create vector of buckets
        // go through list and put cells in buckets
        let s9 = "123456789";
        let ss9 = [s9; 9];
        let sol9 = ss9.join("");

        let mut cells = Cells::from(sol9.as_str());

        let cols = collect_columns(cells);
        println!("{:?}", cols);
        
        cells.set_at(0,8u8);
        println!("{:?}", cols);
    }
}
