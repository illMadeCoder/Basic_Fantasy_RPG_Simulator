use crate::point::Point;

#[derive(Debug)]
pub struct Grid<T> {
    pub vec: Vec<Option<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let vec: Vec<Option<T>> = (0..width * height).map(|_| None).collect();
        Grid { vec, width, height }
    }

    fn point_to_index(&self, Point { x, y }: Point) -> usize {
        (x + y * self.width as i32) as usize
    }

    pub fn insert(&mut self, element: T, p: Point) {
        let i = self.point_to_index(p);
        self.vec[i] = Some(element);
    }

    pub fn remove(&mut self, p: Point) {
        let i = self.point_to_index(p);
        self.vec[i] = None;
    }

    /// # safety
    /// vector index is safe due to pre total population
    pub fn get(&self, p: Point) -> Option<&T> {
        self.vec[self.point_to_index(p)].as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_grid_new_0d() {
        let g: Grid<i32> = Grid::new(0, 0);
        assert_eq!(g.vec, vec![])
    }

    #[test]
    fn test_grid_new_1d() {
        let g: Grid<i32> = Grid::new(1, 1);
        assert_eq!(g.vec, vec![None])
    }

    #[test]
    fn test_grid_new_2d() {
        let g: Grid<i32> = Grid::new(1, 2);
        assert_eq!(g.vec, vec![None, None])
    }

    #[test]
    fn test_grid_insert() {
        let mut g: Grid<i32> = Grid::new(1, 1);
        let p = Point { x: 0, y: 0 };
        let val = 1;
        g.insert(val, p);
        assert_eq!(g.vec, vec![Some(val)])
    }

    #[test]
    fn test_grid_get() {
        let mut g: Grid<i32> = Grid::new(1, 1);
        let p = Point { x: 0, y: 0 };
        let val = 1;
        g.insert(val, p);
        assert_eq!(*g.get(p).unwrap(), 1)
    }

    #[test]
    fn test_grid_remove() {
        let mut g: Grid<i32> = Grid::new(1, 1);
        let p = Point { x: 0, y: 0 };
        let val = 1;
        g.insert(val, p);
        g.remove(p);
        assert_eq!(g.vec, vec![None])
    }

    #[test]
    fn test_grid_many_insert() {
        let mut g: Grid<i32> = Grid::new(3, 3);
        let val = 1;

        let p_top_left = Point { x: 0, y: 0 };
        let p_top_right = Point { x: 2, y: 0 };
        let p_center_center = Point { x: 1, y: 1 };
        let p_bot_right = Point { x: 2, y: 2 };

        let v = vec![Some(val), None, None, None, None, None, None, None, None];
        g.insert(val, p_top_left);
        assert_eq!(g.vec, v);

        let v = vec![
            Some(val),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(val),
        ];
        g.insert(val, p_bot_right);
        assert_eq!(g.vec, v);

        let v = vec![
            Some(val),
            None,
            None,
            None,
            Some(val),
            None,
            None,
            None,
            Some(val),
        ];
        g.insert(val, p_center_center);
        assert_eq!(g.vec, v);

        let v = vec![
            Some(val),
            None,
            Some(val),
            None,
            Some(val),
            None,
            None,
            None,
            Some(val),
        ];
        g.insert(val, p_top_right);
        assert_eq!(g.vec, v);

        let v = vec![
            Some(val),
            None,
            None,
            None,
            Some(val),
            None,
            None,
            None,
            Some(val),
        ];
        g.remove(p_top_right);
        assert_eq!(g.vec, v);
    }
}
