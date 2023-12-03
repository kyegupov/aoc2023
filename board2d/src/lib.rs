use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Ord, Debug)]
pub struct XY(pub isize, pub isize);

impl PartialOrd for XY {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1).then(self.0.cmp(&other.0)))
    }
}

#[derive(Default)]
pub struct Board2D<T>(pub BTreeMap<isize, BTreeMap<isize, T>>);

impl XY {
    pub fn neighbors8(&self) -> Vec<XY> {
        (-1..=1)
            .flat_map(|dy| {
                (-1..=1)
                    .filter(move |dx| *dx != 0 || dy != 0)
                    .map(move |dx| XY(self.0 + dx, self.1 + dy))
            })
            .collect()
    }
}

impl<T> Board2D<T> {
    pub fn get(&self, index: XY) -> Option<&T> {
        self.0.get(&index.1).and_then(|row| row.get(&index.0))
    }
    pub fn set(&mut self, index: XY, value: T) {
        self.0.entry(index.1).and_modify(|row| {
            row.insert(index.0, value);
        });
    }
}

pub fn chars_board_from(iter: &mut dyn Iterator<Item = &str>) -> Board2D<char> {
    let mut res = Board2D::default();
    let mut y = 0;
    for line in iter {
        res.0.insert(
            y,
            line.chars()
                .enumerate()
                .map(|(k, v)| (k as isize, v))
                .collect(),
        );
        y += 1;
    }
    res
}
