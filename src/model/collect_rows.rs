use crate::model::cells::Cells;
use crate::model::row::Row;
use crate::model::rows::Rows;

pub fn collect_rows(cells: &Cells) -> Rows {
    let v = cells.values();
    let c = v.chunks(9);
    let mut r = Rows::new();
    for chunk in c {
         r.add_row(Row::new(chunk.to_vec()));
    }
    r
}

#[cfg(test)]
mod tests {
    use std::ops::Index;
    use crate::model::cells::Cells;
    use crate::model::collect_rows::collect_rows;

    #[test]
    fn test_collect_rows(){
        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";

        let c = Cells::from(solution);
        let rows = collect_rows(&c);
        let v = rows.values();
        assert_eq!(v.len(), 9);
        println!("{:?}", v.index(0));
    }
}

