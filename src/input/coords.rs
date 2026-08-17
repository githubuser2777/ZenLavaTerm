//! Coordinate transformation between terminal grid cells and continuous simulation space.

/// Converts terminal character cell coordinates `(col, row)` into continuous normalized $[0.0, 1.0] \times [0.0, 1.0]$ simulation coordinates.
///
/// In the terminal grid:
/// - `col = 0` is the leftmost column, `col = cols - 1` is the rightmost.
/// - `row = 0` is the topmost row, `row = rows - 1` is the bottom row.
///
/// In simulation space:
/// - `sim_x = 0.0` is the left boundary, `sim_x = 1.0` is the right boundary.
/// - `sim_y = 0.0` is the bottom boundary (heat plate), `sim_y = 1.0` is the top boundary (cooling zone).
#[inline]
pub fn terminal_to_sim_coords(col: u16, row: u16, cols: u16, rows: u16) -> (f32, f32) {
    if cols == 0 || rows == 0 {
        return (0.5, 0.5);
    }

    let clamped_col = col.min(cols.saturating_sub(1));
    let clamped_row = row.min(rows.saturating_sub(1));

    let sim_x = (clamped_col as f32 + 0.5) / cols as f32;
    let sim_y = 1.0 - (clamped_row as f32 + 0.5) / rows as f32;

    (sim_x.clamp(0.0, 1.0), sim_y.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_mapping_center() {
        let (x, y) = terminal_to_sim_coords(40, 12, 80, 24);
        assert!((x - 0.50625).abs() < 1e-4);
        assert!((y - 0.47916).abs() < 1e-4);
    }

    #[test]
    fn test_coordinate_mapping_corners() {
        // Top-left cell (col=0, row=0) -> sim_x near 0.0, sim_y near 1.0 (cooling top)
        let (tl_x, tl_y) = terminal_to_sim_coords(0, 0, 100, 50);
        assert!((tl_x - 0.005).abs() < 1e-4);
        assert!((tl_y - 0.99).abs() < 1e-4);

        // Bottom-right cell (col=99, row=49) -> sim_x near 1.0, sim_y near 0.0 (heat bottom)
        let (br_x, br_y) = terminal_to_sim_coords(99, 49, 100, 50);
        assert!((br_x - 0.995).abs() < 1e-4);
        assert!((br_y - 0.01).abs() < 1e-4);
    }

    #[test]
    fn test_zero_dimensions_graceful_fallback() {
        let (x, y) = terminal_to_sim_coords(10, 10, 0, 0);
        assert_eq!(x, 0.5);
        assert_eq!(y, 0.5);
    }

    #[test]
    fn test_out_of_bounds_clamping() {
        let (x, y) = terminal_to_sim_coords(120, 80, 80, 24);
        assert!(x <= 1.0);
        assert!(y >= 0.0);
    }
}
