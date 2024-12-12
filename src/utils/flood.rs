use crate::utils::grid::{DenseGrid, XY};
use std::collections::HashSet;

impl<T: Eq + Copy> DenseGrid<T> {
    pub fn flood_fill(&self, start: XY) -> HashSet<XY> {
        self.flood_fill_by(start, |v| *v)
    }
}

impl<T: Copy> DenseGrid<T> {
    pub fn flood_fill_by<K: Eq>(&self, start: XY, key_func: impl Fn(&T) -> K) -> HashSet<XY> {
        let initial_value = key_func(
            self.at(start.as_tuple())
                .expect("invalid starting position"),
        );

        let mut cells = HashSet::new();
        let mut pending = HashSet::new();
        pending.insert(start);

        while let Some(pos) = pending.iter().cloned().next() {
            pending.remove(&pos);
            cells.insert(pos);
            pos.cardinal_neighbours().for_each(|p| {
                if let Some(value) = self.at(p.as_tuple()) {
                    if key_func(value) == initial_value
                        && !pending.contains(&p)
                        && !cells.contains(&p)
                    {
                        pending.insert(p);
                    }
                }
            })
        }

        cells
    }
}
