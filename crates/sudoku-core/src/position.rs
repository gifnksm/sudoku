//! Board position and coordinate utilities.

/// Board position (x, y) where x is column and y is row.
///
/// Both coordinates are in the range 0-8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: u8,
    y: u8,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare by index order: y * 9 + x
        // This ensures ordering matches PositionSemantics::to_index
        (self.y * 9 + self.x).cmp(&(other.y * 9 + other.x))
    }
}

impl Position {
    /// Creates a new position from column and row coordinates.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is greater than or equal to 9.
    #[must_use]
    pub const fn new(x: u8, y: u8) -> Self {
        assert!(x < 9 && y < 9);
        Self { x, y }
    }

    /// Creates a position from box index and cell index within that box.
    ///
    /// # Panics
    ///
    /// Panics if `box_index` or `cell_index` is greater than or equal to 9.
    #[must_use]
    pub const fn from_box(box_index: u8, cell_index: u8) -> Self {
        assert!(box_index < 9 && cell_index < 9);
        let origin = box_top_left(box_index);
        Self::new(origin.x + cell_index % 3, origin.y + cell_index / 3)
    }

    /// Returns the column (x coordinate) of this position.
    #[must_use]
    pub const fn x(self) -> u8 {
        self.x
    }

    /// Returns the row (y coordinate) of this position.
    #[must_use]
    pub const fn y(self) -> u8 {
        self.y
    }

    /// Returns the box index (0-8) that this position belongs to.
    #[must_use]
    pub const fn box_index(&self) -> u8 {
        (self.y / 3) * 3 + (self.x / 3)
    }

    /// Returns the relative position (0-8) within the box.
    #[must_use]
    pub const fn box_cell_index(&self) -> u8 {
        (self.y % 3) * 3 + (self.x % 3)
    }
}

/// Returns the top-left position of the specified box.
///
/// # Panics
///
/// Panics if `box_index` is greater than or equal to 9.
#[must_use]
pub const fn box_top_left(box_index: u8) -> Position {
    assert!(box_index < 9);
    Position::new((box_index % 3) * 3, (box_index / 3) * 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_position() {
        let pos = Position::new(3, 5);
        assert_eq!(pos.x(), 3);
        assert_eq!(pos.y(), 5);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_new_position_x_too_large() {
        let _ = Position::new(9, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_new_position_y_too_large() {
        let _ = Position::new(0, 9);
    }

    #[test]
    fn test_box_index() {
        // Box 0 (top-left)
        assert_eq!(Position::new(0, 0).box_index(), 0);
        assert_eq!(Position::new(1, 1).box_index(), 0);
        assert_eq!(Position::new(2, 2).box_index(), 0);

        // Box 1 (top-center)
        assert_eq!(Position::new(3, 0).box_index(), 1);
        assert_eq!(Position::new(4, 1).box_index(), 1);
        assert_eq!(Position::new(5, 2).box_index(), 1);

        // Box 2 (top-right)
        assert_eq!(Position::new(6, 0).box_index(), 2);
        assert_eq!(Position::new(7, 1).box_index(), 2);
        assert_eq!(Position::new(8, 2).box_index(), 2);

        // Box 3 (middle-left)
        assert_eq!(Position::new(0, 3).box_index(), 3);
        assert_eq!(Position::new(1, 4).box_index(), 3);
        assert_eq!(Position::new(2, 5).box_index(), 3);

        // Box 4 (center)
        assert_eq!(Position::new(3, 3).box_index(), 4);
        assert_eq!(Position::new(4, 4).box_index(), 4);
        assert_eq!(Position::new(5, 5).box_index(), 4);

        // Box 5 (middle-right)
        assert_eq!(Position::new(6, 3).box_index(), 5);
        assert_eq!(Position::new(7, 4).box_index(), 5);
        assert_eq!(Position::new(8, 5).box_index(), 5);

        // Box 6 (bottom-left)
        assert_eq!(Position::new(0, 6).box_index(), 6);
        assert_eq!(Position::new(1, 7).box_index(), 6);
        assert_eq!(Position::new(2, 8).box_index(), 6);

        // Box 7 (bottom-center)
        assert_eq!(Position::new(3, 6).box_index(), 7);
        assert_eq!(Position::new(4, 7).box_index(), 7);
        assert_eq!(Position::new(5, 8).box_index(), 7);

        // Box 8 (bottom-right)
        assert_eq!(Position::new(6, 6).box_index(), 8);
        assert_eq!(Position::new(7, 7).box_index(), 8);
        assert_eq!(Position::new(8, 8).box_index(), 8);
    }

    #[test]
    fn test_box_cell_index() {
        // Top-left of box
        assert_eq!(Position::new(0, 0).box_cell_index(), 0);
        assert_eq!(Position::new(3, 3).box_cell_index(), 0);
        assert_eq!(Position::new(6, 6).box_cell_index(), 0);

        // Top-center of box
        assert_eq!(Position::new(1, 0).box_cell_index(), 1);
        assert_eq!(Position::new(4, 3).box_cell_index(), 1);

        // Top-right of box
        assert_eq!(Position::new(2, 0).box_cell_index(), 2);
        assert_eq!(Position::new(5, 3).box_cell_index(), 2);

        // Middle-left of box
        assert_eq!(Position::new(0, 1).box_cell_index(), 3);
        assert_eq!(Position::new(3, 4).box_cell_index(), 3);

        // Center of box
        assert_eq!(Position::new(1, 1).box_cell_index(), 4);
        assert_eq!(Position::new(4, 4).box_cell_index(), 4);

        // Middle-right of box
        assert_eq!(Position::new(2, 1).box_cell_index(), 5);
        assert_eq!(Position::new(5, 4).box_cell_index(), 5);

        // Bottom-left of box
        assert_eq!(Position::new(0, 2).box_cell_index(), 6);
        assert_eq!(Position::new(3, 5).box_cell_index(), 6);

        // Bottom-center of box
        assert_eq!(Position::new(1, 2).box_cell_index(), 7);
        assert_eq!(Position::new(4, 5).box_cell_index(), 7);

        // Bottom-right of box
        assert_eq!(Position::new(2, 2).box_cell_index(), 8);
        assert_eq!(Position::new(5, 5).box_cell_index(), 8);
    }

    #[test]
    fn test_from_box() {
        // Box 0 cells
        assert_eq!(Position::from_box(0, 0), Position::new(0, 0));
        assert_eq!(Position::from_box(0, 1), Position::new(1, 0));
        assert_eq!(Position::from_box(0, 2), Position::new(2, 0));
        assert_eq!(Position::from_box(0, 3), Position::new(0, 1));
        assert_eq!(Position::from_box(0, 4), Position::new(1, 1));
        assert_eq!(Position::from_box(0, 8), Position::new(2, 2));

        // Box 4 (center) cells
        assert_eq!(Position::from_box(4, 0), Position::new(3, 3));
        assert_eq!(Position::from_box(4, 4), Position::new(4, 4));
        assert_eq!(Position::from_box(4, 8), Position::new(5, 5));

        // Box 8 (bottom-right) cells
        assert_eq!(Position::from_box(8, 0), Position::new(6, 6));
        assert_eq!(Position::from_box(8, 4), Position::new(7, 7));
        assert_eq!(Position::from_box(8, 8), Position::new(8, 8));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_from_box_invalid_box_index() {
        let _ = Position::from_box(9, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_from_box_invalid_cell_index() {
        let _ = Position::from_box(0, 9);
    }

    #[test]
    fn test_box_top_left() {
        assert_eq!(box_top_left(0), Position::new(0, 0));
        assert_eq!(box_top_left(1), Position::new(3, 0));
        assert_eq!(box_top_left(2), Position::new(6, 0));
        assert_eq!(box_top_left(3), Position::new(0, 3));
        assert_eq!(box_top_left(4), Position::new(3, 3));
        assert_eq!(box_top_left(5), Position::new(6, 3));
        assert_eq!(box_top_left(6), Position::new(0, 6));
        assert_eq!(box_top_left(7), Position::new(3, 6));
        assert_eq!(box_top_left(8), Position::new(6, 6));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_box_top_left_invalid() {
        let _ = box_top_left(9);
    }

    #[test]
    fn test_from_box_roundtrip() {
        // Test that from_box and box_index/box_cell_index are inverses
        for box_index in 0..9 {
            for cell_index in 0..9 {
                let pos = Position::from_box(box_index, cell_index);
                assert_eq!(pos.box_index(), box_index);
                assert_eq!(pos.box_cell_index(), cell_index);
            }
        }
    }

    #[test]
    fn test_ord_matches_index_order() {
        // Verify that Position ordering matches index order (y * 9 + x)

        // (1, 0) has index 1, (0, 1) has index 9
        // So (1, 0) should be < (0, 1)
        assert!(Position::new(1, 0) < Position::new(0, 1));

        // (8, 0) has index 8, (0, 1) has index 9
        assert!(Position::new(8, 0) < Position::new(0, 1));

        // (0, 0) < (1, 0) < (2, 0) < ... < (8, 0) < (0, 1) < (1, 1) < ...
        assert!(Position::new(0, 0) < Position::new(1, 0));
        assert!(Position::new(1, 0) < Position::new(2, 0));
        assert!(Position::new(8, 0) < Position::new(0, 1));
        assert!(Position::new(0, 1) < Position::new(1, 1));

        // Last position (8, 8) should be greater than all others
        assert!(Position::new(8, 8) > Position::new(0, 0));
        assert!(Position::new(8, 8) > Position::new(7, 8));

        // Test all positions are in index order
        let mut positions = Vec::new();
        for y in 0..9 {
            for x in 0..9 {
                positions.push(Position::new(x, y));
            }
        }

        for i in 0..positions.len() - 1 {
            assert!(
                positions[i] < positions[i + 1],
                "Position at index {} should be < position at index {}",
                i,
                i + 1
            );
        }
    }

    #[test]
    fn test_all_positions_have_valid_box_index() {
        for y in 0..9 {
            for x in 0..9 {
                let pos = Position::new(x, y);
                let box_idx = pos.box_index();
                assert!(
                    box_idx < 9,
                    "Invalid box index {box_idx} for position ({x}, {y})"
                );
            }
        }
    }

    #[test]
    fn test_all_positions_have_valid_box_cell_index() {
        for y in 0..9 {
            for x in 0..9 {
                let pos = Position::new(x, y);
                let cell_idx = pos.box_cell_index();
                assert!(
                    cell_idx < 9,
                    "Invalid cell index {cell_idx} for position ({x}, {y})"
                );
            }
        }
    }
}
