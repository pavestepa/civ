use crate::HexCoordinate;

/// Iterate all axial hex coordinates in a rectangular map `[0, width) × [0, height)`.
pub fn rect_coordinates(width: u32, height: u32) -> impl Iterator<Item = HexCoordinate> {
    (0..height as i32).flat_map(move |r| (0..width as i32).map(move |q| HexCoordinate::new(q, r)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_map_count() {
        assert_eq!(rect_coordinates(3, 3).count(), 9);
    }
}
