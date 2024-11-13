use crate::model::region::Region;

#[derive(Clone, PartialEq, Debug)]
pub struct Regions {
    values: [Region;9]
}
impl Regions {
    pub fn new() -> Self {
        Regions {
            values: [Region::new();9]
        }
    }
    pub fn with_regions(regions:[Region;9]) -> Self {
        Regions {
            values: regions
        }
    }
    pub fn add_region(&mut self,region: Region) {
       todo!();
    }
}