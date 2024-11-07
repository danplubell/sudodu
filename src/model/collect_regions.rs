use crate::model::cells::Cells;
use crate::model::region::Region;
use crate::model::regions::Regions;

pub fn collect_regions(cells: &Cells) -> Regions {
    let roots:[usize;9] = [0, 3, 6, 27, 30, 33, 57, 60, 63];
    let mut regions = Regions::new();
    for (i, r) in roots.iter().enumerate() {
        let end = r + 3;
        let start = *r;
        let mut region = Region::new();
        for z in start..end {
            let row = r + z * 9;
            for j in 0..3 {
                let idx:usize = r + j;
                println!("{:?}",cells.values()[idx]);
                region.add_cell(&cells.values()[idx])
            }
        }
        regions.add_region(region);
    }
    regions
}

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;
    use crate::model::region::Region;
    use crate::model::regions::Regions;

    fn sequence(n: u32) -> u32 {
        let within_group = n % 3 * 3; // Handles the +3 steps within groups
        let group_jumps = n / 3 * 27; // Handles the +21 jumps between groups
        within_group + group_jumps
    }

    // Usage examples:
    // sequence(0) => 0
    // sequence(1) => 3
    // sequence(2) => 6
    // sequence(3) => 27
    // sequence(4) => 30
    // sequence(5) => 33
    // sequence(6) => 54
    // sequence(7) => 57
    // sequence(8) => 60
    #[test]
    fn test_collect_regions() {
        let s9 = "123456789";
        let ss9 = [s9; 9];
        let sol9 = ss9.join("");

        let cells = Cells::from(sol9.as_str());
        let region: [[usize; 3]; 9] = [
            [0, 9, 18],
            [3, 12, 21],
            [6, 15, 24],
            [27, 36, 45],
            [30, 39, 48],
            [33, 42, 51],
            [57, 66, 74],
            [60, 69, 78],
            [63, 72, 81],
        ];
        /*
        let roots:[usize;9] = [0, 3, 6, 27, 30, 33, 57, 60, 63];
        let mut regions = Regions::new();
        for (i, r) in roots.iter().enumerate() {
            let end = r + 3;
            let start = *r;
            let mut region = Region::new();
            for z in start..end {
                let row = r + z * 9;
                for j in 0..3 {
                    let idx:usize = r + j;
                    println!("{:?}",cells.values()[idx]);
                    region.add_cell(&cells.values()[idx])
                }
            }
            regions.add_region(region);
        }
        
         */
        /*
        for r in region {
            for r1 in r {
                cells
                    .values()
                    .iter()
                    .take(3)
                    .for_each(|e| println!("{:?}", e));
            }
        }
        
         */
    }
}

/*
take 3 columns,
take 3 rows of first column
 0   1   2
318 457 962 572 986 143
946 312 578

639178425
157294836
284563791
425731689
761829354
893645217";
 */
