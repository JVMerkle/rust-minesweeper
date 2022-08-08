use super::Position;

pub struct NeighbourIter {
    origin: Position,
    i: usize,
    distance: i32,
}

impl NeighbourIter {
    pub fn new(origin: Position) -> Self {
        NeighbourIter {
            origin,
            i: 0,
            distance: 1,
        }
    }
}

impl Iterator for NeighbourIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbour_offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*******/ (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        let offset = neighbour_offsets[self.i];

        let pos = Position::from(self.origin.x + offset.0 * self.distance, self.origin.y + offset.1 * self.distance);

        self.i += 1;
        if self.i >= neighbour_offsets.len() {
            self.i = 0;
            self.distance += 1;
        }

        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_values() {
        let neighbours: Vec<Position> = NeighbourIter::new(Position::from(0, 0)).take(20).collect();
        let mut unique = neighbours.clone();
        unique.sort();
        unique.dedup();

        assert_eq!(neighbours.len(), 20);
        assert_eq!(neighbours.len(), unique.len());
    }
}
