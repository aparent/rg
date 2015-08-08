#[derive(Debug)]
pub struct ItemType {
    pub name : String,
}

#[derive(Debug)]
pub struct LootTable<'a>{
    prob_vec : Vec<(i64,f64)>,
    type_table : &'a Vec<ItemType>,
}

#[derive(Debug)]
pub struct LTBuilder<'a> {
    type_table : &'a Vec<ItemType>,
}

impl<'a> LTBuilder<'a> {
    pub fn new(type_table : &Vec<ItemType> ) -> LTBuilder {
        LTBuilder { type_table: type_table }
    }
    //pub fn mk_loot_table(&self, String: name, prob: f64
    pub fn mk_loot_table_id(&self, item_id: i64, prob: f64 ) -> LootTable {
        LootTable{ prob_vec: vec![(item_id, prob)], type_table: self.type_table }
    }
}
