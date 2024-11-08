use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::region::Region;
use crate::model::regions::Regions;

pub fn collect_regions(cells: &Cells) -> Regions {
    let roots:[usize;8] = [0, 3, 6, 27, 30, 33, 57, 60];
    let mut regions = Regions::new();
    for r in roots.iter() {
        
        let start = *r;
        let end = r + 18;
        let mut t = Vec::new();
        for i in (start..=end).step_by(9usize) {
            let start = i;
            let end = i + 3;
            for j in start..end {
                println!("{}",j);
                t.push(Cell::new(cells.get_at(j).unwrap().value))
            }
        }
        regions.add_region(Region::new(t));
    }
    regions
}

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;
    use crate::model::collect_regions::collect_regions;

    #[test]
    fn test_collect_regions() {
        let s9 = "123456789";
        let ss9 = [s9; 9];
        let sol9 = ss9.join("");

        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let cells = Cells::from(solution);
        let r = collect_regions(&cells);
        println!("{:?}", r);
    }
}
