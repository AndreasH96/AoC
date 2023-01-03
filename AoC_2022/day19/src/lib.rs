use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: usize,
    pub price_list: HashMap<usize, HashMap<usize, u32>>,
    pub max_costs: Vec<u32>,
    pub max_geodes: usize,
    pub min_geode_creation: u32,
}

impl Blueprint {
    pub fn new(id: usize, price_list: HashMap<usize, HashMap<usize, u32>>) -> Blueprint {
        let mut max_costs: Vec<u32> = vec![0, 0, 0, 0];
        for i in 0..4 {
            let current_prices = price_list.get(&i).unwrap();

            for j in 0..4 {
                max_costs[j] = if current_prices[&j] > max_costs[j] {
                    current_prices[&j]
                } else {
                    max_costs[j]
                }
            }
        }

        Blueprint {
            id,
            price_list,
            max_costs,
            max_geodes: 0,
            min_geode_creation: 24,
        }
    }

    pub fn update_max_geodes(&mut self, geodes: usize) {
        if geodes > self.max_geodes {
            self.max_geodes = geodes;
        }
    }
    pub fn update_min_geode_creation(&mut self, min_geode_creation: u32) {
        if min_geode_creation < self.min_geode_creation {
            self.min_geode_creation = min_geode_creation;
        }
    }
 
}
