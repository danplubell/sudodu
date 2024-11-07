use crate::model::region::Region;

#[derive(Clone, PartialEq, Debug)]
pub struct Regions {
    values: Vec<Region>
}
impl Regions {
    pub fn new() -> Self {
        Regions {
            values: Vec::new()
        }
    }
    pub fn add_region(&mut self,region: Region) {
        self.values.push(region);
    }
}