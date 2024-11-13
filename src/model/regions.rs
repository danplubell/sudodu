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
    pub fn add_region_at(&mut self,region: Region, idx: usize) {
       self.values[idx] = region;
    }
    pub fn get_at(&self,index:usize) -> Region {
        self.values[index]
    }
    pub fn iter(&self) -> impl Iterator<Item = &Region> {
        self.values.iter()
    }
}