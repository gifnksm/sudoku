//! Board position types.

use crate::containers::Array9;
use crate::index::CellIndexSemantics;

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
    /// All 81 positions on the Sudoku board, in index order.
    ///
    /// The positions are ordered by row-major order (y * 9 + x):
    /// - Index 0: (0, 0) - top-left
    /// - Index 1: (1, 0)
    /// - ...
    /// - Index 8: (8, 0) - top-right
    /// - Index 9: (0, 1) - second row, first column
    /// - ...
    /// - Index 80: (8, 8) - bottom-right
    ///
    /// This order matches `PositionSemantics::to_index` and `Position::Ord`.
    pub const ALL: [Position; 81] = {
        let mut arr = [Position::new(0, 0); 81];
        let mut i = 0;
        while i < 81 {
            arr[i as usize] = Position::new(i % 9, i / 9);
            i += 1;
        }
        arr
    };

    /// All positions in each row, indexed by row number (0-8).
    ///
    /// `ROWS[y]` contains all 9 positions in row `y`, ordered by column (x = 0..9).
    ///
    /// # Example
    ///
    /// ```
    /// # use sudoku_core::Position;
    /// // Process all positions in row 3
    /// for pos in Position::ROWS[3] {
    ///     assert_eq!(pos.y(), 3);
    /// }
    /// ```
    pub const ROWS: Array9<[Position; 9], CellIndexSemantics> = {
        let mut rows = [[Position::new(0, 0); 9]; 9];
        let mut y = 0;
        while y < 9 {
            let mut x = 0;
            while x < 9 {
                rows[y as usize][x as usize] = Position::new(x, y);
                x += 1;
            }
            y += 1;
        }
        Array9::from_array(rows)
    };

    /// All positions in each column, indexed by column number (0-8).
    ///
    /// `COLUMNS[x]` contains all 9 positions in column `x`, ordered by row (y = 0..9).
    ///
    /// # Example
    ///
    /// ```
    /// # use sudoku_core::Position;
    /// // Process all positions in column 5
    /// for pos in Position::COLUMNS[5] {
    ///     assert_eq!(pos.x(), 5);
    /// }
    /// ```
    pub const COLUMNS: Array9<[Position; 9], CellIndexSemantics> = {
        let mut columns = [[Position::new(0, 0); 9]; 9];
        let mut x = 0;
        while x < 9 {
            let mut y = 0;
            while y < 9 {
                columns[x as usize][y as usize] = Position::new(x, y);
                y += 1;
            }
            x += 1;
        }
        Array9::from_array(columns)
    };

    /// All positions in each 3×3 box, indexed by box number (0-8).
    ///
    /// `BOXES[box_index]` contains all 9 positions in that box, ordered by
    /// cell index within the box (same order as `Position::from_box`).
    ///
    /// Box indices are arranged as:
    /// ```text
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use sudoku_core::Position;
    /// // Process all positions in box 4 (center box)
    /// for pos in Position::BOXES[4] {
    ///     assert_eq!(pos.box_index(), 4);
    /// }
    /// ```
    pub const BOXES: Array9<[Position; 9], CellIndexSemantics> = {
        let mut boxes = [[Position::new(0, 0); 9]; 9];
        let mut box_index = 0;
        while box_index < 9 {
            let mut cell_index = 0;
            while cell_index < 9 {
                boxes[box_index as usize][cell_index as usize] =
                    Position::from_box(box_index, cell_index);
                cell_index += 1;
            }
            box_index += 1;
        }
        Array9::from_array(boxes)
    };

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
        let origin = Self::box_origin(box_index);
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

    /// Returns the top-left position (origin) of the specified box.
    ///
    /// # Panics
    ///
    /// Panics if `box_index` is greater than or equal to 9.
    #[must_use]
    pub const fn box_origin(box_index: u8) -> Self {
        assert!(box_index < 9);
        Self::new((box_index % 3) * 3, (box_index / 3) * 3)
    }
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
    fn test_box_origin() {
        assert_eq!(Position::box_origin(0), Position::new(0, 0));
        assert_eq!(Position::box_origin(1), Position::new(3, 0));
        assert_eq!(Position::box_origin(2), Position::new(6, 0));
        assert_eq!(Position::box_origin(3), Position::new(0, 3));
        assert_eq!(Position::box_origin(4), Position::new(3, 3));
        assert_eq!(Position::box_origin(5), Position::new(6, 3));
        assert_eq!(Position::box_origin(6), Position::new(0, 6));
        assert_eq!(Position::box_origin(7), Position::new(3, 6));
        assert_eq!(Position::box_origin(8), Position::new(6, 6));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_box_origin_invalid() {
        let _ = Position::box_origin(9);
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
        use crate::index::{Index81Semantics, PositionSemantics};

        // Verify that Position ordering matches index order (y * 9 + x)
        // and that the ordering matches PositionSemantics::to_index

        // (1, 0) has index 1, (0, 1) has index 9
        // So (1, 0) should be < (0, 1)
        let pos_1_0 = Position::new(1, 0);
        let pos_0_1 = Position::new(0, 1);
        assert!(pos_1_0 < pos_0_1);
        assert_eq!(PositionSemantics::to_index(pos_1_0).index(), 1);
        assert_eq!(PositionSemantics::to_index(pos_0_1).index(), 9);

        // (8, 0) has index 8, (0, 1) has index 9
        let pos_8_0 = Position::new(8, 0);
        assert!(pos_8_0 < pos_0_1);
        assert_eq!(PositionSemantics::to_index(pos_8_0).index(), 8);

        // (0, 0) < (1, 0) < (2, 0) < ... < (8, 0) < (0, 1) < (1, 1) < ...
        assert!(Position::new(0, 0) < Position::new(1, 0));
        assert!(Position::new(1, 0) < Position::new(2, 0));
        assert!(Position::new(8, 0) < Position::new(0, 1));
        assert!(Position::new(0, 1) < Position::new(1, 1));

        // Last position (8, 8) should be greater than all others
        assert!(Position::new(8, 8) > Position::new(0, 0));
        assert!(Position::new(8, 8) > Position::new(7, 8));
        assert_eq!(PositionSemantics::to_index(Position::new(8, 8)).index(), 80);

        // Test all positions are in index order and match PositionSemantics
        for (expected_index, &pos) in (0u8..).zip(Position::ALL.iter()) {
            let actual_index = PositionSemantics::to_index(pos).index();
            assert_eq!(
                actual_index, expected_index,
                "Position::ALL[{expected_index}] = {pos:?} should have index {expected_index}, got {actual_index}"
            );

            // Verify ordering
            if expected_index > 0 {
                let prev_pos = Position::ALL[usize::from(expected_index - 1)];
                assert!(
                    prev_pos < pos,
                    "Position at index {} should be < position at index {expected_index}",
                    expected_index - 1
                );
            }
        }
    }

    #[test]
    fn test_rows_constant() {
        // Verify ROWS contains all positions for each row
        for y in 0..9 {
            for (x, &pos) in (0u8..).zip(Position::ROWS[y].iter()) {
                assert_eq!(pos.x(), x);
                assert_eq!(pos.y(), y);
            }
        }
    }

    #[test]
    fn test_columns_constant() {
        // Verify COLUMNS contains all positions for each column
        for x in 0..9 {
            for (y, &pos) in (0u8..).zip(Position::COLUMNS[x].iter()) {
                assert_eq!(pos.x(), x);
                assert_eq!(pos.y(), y);
            }
        }
    }

    #[test]
    fn test_boxes_constant() {
        // Verify BOXES contains all positions for each box
        for box_index in 0..9 {
            for (cell_index, &pos) in (0u8..).zip(Position::BOXES[box_index].iter()) {
                assert_eq!(pos.box_index(), box_index);
                assert_eq!(pos.box_cell_index(), cell_index);
                // Also verify it matches from_box
                assert_eq!(pos, Position::from_box(box_index, cell_index));
            }
        }
    }

    #[test]
    fn test_rows_no_duplicates() {
        use std::collections::HashSet;
        for y in 0..9 {
            let set: HashSet<Position> = Position::ROWS[y].iter().copied().collect();
            assert_eq!(set.len(), 9, "Row {y} should contain 9 unique positions");
        }
    }

    #[test]
    fn test_columns_no_duplicates() {
        use std::collections::HashSet;
        for x in 0..9 {
            let set: HashSet<Position> = Position::COLUMNS[x].iter().copied().collect();
            assert_eq!(set.len(), 9, "Column {x} should contain 9 unique positions");
        }
    }

    #[test]
    fn test_boxes_no_duplicates() {
        use std::collections::HashSet;
        for box_index in 0..9 {
            let set: HashSet<Position> = Position::BOXES[box_index].iter().copied().collect();
            assert_eq!(
                set.len(),
                9,
                "Box {box_index} should contain 9 unique positions"
            );
        }
    }

    #[test]
    fn test_all_array_matches_index_order() {
        use crate::index::{Index81Semantics, PositionSemantics};

        // Verify that Position::ALL[i] corresponds to Index81(i) via PositionSemantics
        for (i, &pos) in (0u8..).zip(Position::ALL.iter()) {
            let index = PositionSemantics::to_index(pos);
            assert_eq!(
                index.index(),
                i,
                "Position::ALL[{i}] = {pos:?} should convert to Index81({i}), got Index81({})",
                index.index()
            );

            // Also verify the position has the expected coordinates
            let expected_x = i % 9;
            let expected_y = i / 9;
            assert_eq!(
                pos.x(),
                expected_x,
                "Position::ALL[{i}] should have x={expected_x}, got {}",
                pos.x()
            );
            assert_eq!(
                pos.y(),
                expected_y,
                "Position::ALL[{i}] should have y={expected_y}, got {}",
                pos.y()
            );
        }
    }

    #[test]
    fn test_all_array_length() {
        assert_eq!(
            Position::ALL.len(),
            81,
            "Position::ALL should contain exactly 81 positions"
        );
    }

    #[test]
    fn test_all_array_no_duplicates() {
        use std::collections::HashSet;
        let set: HashSet<Position> = Position::ALL.iter().copied().collect();
        assert_eq!(
            set.len(),
            81,
            "Position::ALL should contain 81 unique positions"
        );
    }

    #[test]
    fn test_all_positions_have_valid_box_index() {
        for pos in Position::ALL {
            let box_idx = pos.box_index();
            assert!(
                box_idx < 9,
                "Invalid box index {box_idx} for position ({}, {})",
                pos.x(),
                pos.y()
            );
        }
    }

    #[test]
    fn test_all_positions_have_valid_box_cell_index() {
        for pos in Position::ALL {
            let cell_idx = pos.box_cell_index();
            assert!(
                cell_idx < 9,
                "Invalid cell index {cell_idx} for position ({}, {})",
                pos.x(),
                pos.y()
            );
        }
    }
}
