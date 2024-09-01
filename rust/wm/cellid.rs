use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// An identifier for a cell in the World Mercator (WM) coordinate system.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(C)]
pub struct WMCellId {
    /// the id contains the face, s, and t components
    pub id: u64,
}
impl WMCellId {
    /// Creates a new WMCellId from the given id.
    pub fn new(id: u64) -> Self {
        WMCellId { id }
    }

    /// Creates a new WMCellId from the zoom level and (x, y) coordinates.
    /// The id is encoded such that the first 6 bits store the zoom level,
    /// and the next 29 bits each are used to store the x and y coordinates.
    pub fn from_zoom_xy(zoom: u8, x: u32, y: u32) -> Self {
        debug_assert!(zoom < 30, "Zoom level must be less than 30.");

        // Encode the zoom level into the first 6 bits
        let zoom_part = (zoom as u64) << 58;
        // Encode the x coordinate into the next 29 bits
        let x_part = (x as u64 & ((1 << 29) - 1)) << 29;
        // Encode the y coordinate into the remaining 29 bits
        let y_part = y as u64 & ((1 << 29) - 1);
        // Combine all parts into a single u64
        let id = zoom_part | x_part | y_part;

        (id).into()
    }

    /// Decode the WMCellId back into (zoom, x, y) components.
    pub fn to_zoom_xy(&self) -> (u8, u32, u32) {
        let zoom = (self.id >> 58) as u8;
        let x = ((self.id >> 29) & ((1 << 29) - 1)) as u32;
        let y = (self.id & ((1 << 29) - 1)) as u32;

        (zoom, x, y)
    }

    /// grab the quad children of the current cell
    pub fn children(&self) -> [Self; 4] {
        let (zoom, x, y) = self.to_zoom_xy();
        let child_zoom = zoom + 1;
        let child_x = x * 2;
        let child_y = y * 2;
        [
            WMCellId::from_zoom_xy(child_zoom, child_x, child_y),
            WMCellId::from_zoom_xy(child_zoom, child_x + 1, child_y),
            WMCellId::from_zoom_xy(child_zoom, child_x, child_y + 1),
            WMCellId::from_zoom_xy(child_zoom, child_x + 1, child_y + 1),
        ]
    }

    /// grab the tiles next to the current tiles zoom-x-y
    /// only include adjacent tiles, not diagonal.
    /// If includeOutOfBounds set to true, it will include out of bounds tiles
    /// on the x-axis
    pub fn neighbors(&self, include_out_of_bounds: bool) -> Vec<Self> {
        let mut neighbors: BTreeSet<Self> = BTreeSet::new();
        let (zoom, mut x, y) = self.to_zoom_xy();
        let size: u32 = 1 << zoom;
        let x_out_of_bounds = x >= size;

        if x > 0 || include_out_of_bounds {
            x = if x == 0 { size } else { x };
            neighbors.insert(Self::from_zoom_xy(zoom, x - 1, y));
        }
        if x + 1 < size || include_out_of_bounds {
            neighbors.insert(Self::from_zoom_xy(zoom, x + 1, y));
        }
        if !x_out_of_bounds && y > 0 {
            neighbors.insert(Self::from_zoom_xy(zoom, x, y - 1));
        }
        if !x_out_of_bounds && y + 1 < size {
            neighbors.insert(Self::from_zoom_xy(zoom, x, y + 1));
        }

        neighbors.into_iter().collect()
    }

    /// Check if the tile is not a real world tile that fits inside the quad tree
    /// Out of bounds tiles exist if the map has `duplicateHorizontally` set to true.
    /// This is useful for filling in the canvas on the x axis instead of leaving it blank.
    pub fn is_out_of_bounds(&self) -> bool {
        let (zoom, x, y) = self.to_zoom_xy();
        let size = 1 << zoom;
        x >= size || y >= size
    }

    /// Given a tile ID, find the "wrapped" tile ID.
    /// It may resolve to itself. This is useful for maps that have
    /// `duplicateHorizontally` set to true. It forces the tile to be
    /// within the bounds of the quad tree.
    pub fn tile_id_wrapped(&self) -> WMCellId {
        let (zoom, x, y) = self.to_zoom_xy();
        let size = 1 << zoom;
        WMCellId::from_zoom_xy(zoom, x % size, y % size)
    }

    /// Given a tileID, find the parent tile
    pub fn parent(&self) -> WMCellId {
        let (zoom, x, y) = self.to_zoom_xy();
        debug_assert!(zoom != 0, "You can not find a parent at the face level.");
        WMCellId::from_zoom_xy(zoom - 1, x >> 1, y >> 1)
    }

    /// convert an id to a zoom-x-y after setting it to a new parent zoom
    pub fn to_zoom_ij(&self, level: Option<u8>) -> (u8, u32, u32) {
        let mut id = *self;
        if let Some(level) = level {
            let (mut curr_zoom, _x, _y) = self.to_zoom_xy();
            while level < curr_zoom {
                id = id.parent();
                curr_zoom -= 1;
            }
        }
        id.to_zoom_xy()
    }

    /// Check if the parentID contains the childID within the sub quads
    pub fn contains(&self, other: &WMCellId) -> bool {
        let (pz, px, py) = self.to_zoom_xy();
        let (cz, cx, cy) = other.to_zoom_xy();
        if pz > cz {
            return false;
        }
        // Calculate the difference of child at the parent's level
        let diff = cz - pz;
        // check if x and y match adjusting child x,y to parent's level
        px == cx >> diff && py == cy >> diff
    }

    /// Given a Tile ID, check if the zoom is 0 or not
    pub fn is_face(&self) -> bool {
        self.level() == 0
    }

    /// Get the zoom from the tile ID
    pub fn level(&self) -> u8 {
        let (zoom, _, _) = self.to_zoom_xy();
        zoom
    }
}
impl From<u64> for WMCellId {
    fn from(value: u64) -> Self {
        WMCellId::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_id() {
        // to id
        assert_eq!(WMCellId::from_zoom_xy(0, 0, 0), WMCellId::new(0));
        assert_eq!(WMCellId::from_zoom_xy(1, 0, 0), WMCellId::new(288230376151711744));
        // from id
        assert_eq!(WMCellId::new(0), WMCellId::from_zoom_xy(0, 0, 0));
        assert_eq!(WMCellId::new(288230376151711744), WMCellId::from_zoom_xy(1, 0, 0));

        // to-from
        assert_eq!(WMCellId::from_zoom_xy(0, 0, 0).to_zoom_xy(), (0, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(1, 0, 0).to_zoom_xy(), (1, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(1, 1, 0).to_zoom_xy(), (1, 1, 0));
        assert_eq!(WMCellId::from_zoom_xy(1, 1, 1).to_zoom_xy(), (1, 1, 1));
        assert_eq!(
            WMCellId::from_zoom_xy(20, 1048575, 1048575).to_zoom_xy(),
            (20, 1048575, 1048575)
        );
        assert_eq!(
            WMCellId::from_zoom_xy(29, (1 << 29) - 1, (1 << 29) - 1).to_zoom_xy(),
            (29, (1 << 29) - 1, (1 << 29) - 1)
        );
    }

    #[test]
    #[should_panic(expected = "Zoom level must be less than 30.")]
    fn from_zoom_invalid_zoom() {
        let _ = WMCellId::from_zoom_xy(40, 0, 0);
    }

    #[test]
    fn zooms_1_to_7() {
        let mut id_cache: BTreeSet<WMCellId> = BTreeSet::new();
        for z in 1..=7 {
            for x in 0..(1 << z) {
                for y in 0..(1 << z) {
                    let id = WMCellId::from_zoom_xy(z, x, y);
                    if id_cache.contains(&id) {
                        panic!("duplicate id {:?}", id);
                    }
                    id_cache.insert(id);
                    let zxy = id.to_zoom_xy();
                    assert_eq!(z, id.level());
                    assert_eq!(zxy, (z, x, y));
                }
            }
        }
    }

    #[test]
    fn children() {
        let id = WMCellId::from_zoom_xy(0, 0, 0);
        let children = id.children();
        assert_eq!(children.len(), 4);
        assert_eq!(
            children,
            [
                WMCellId::new(288230376151711744),
                WMCellId::new(288230376688582656),
                WMCellId::new(288230376151711745),
                WMCellId::new(288230376688582657),
            ]
        );

        let id2 = WMCellId::from_zoom_xy(1, 0, 0);
        let children2 = id2.children();
        assert_eq!(children2.len(), 4);
        assert_eq!(
            children2,
            [
                WMCellId::new(576460752303423488),
                WMCellId::new(576460752840294400),
                WMCellId::new(576460752303423489),
                WMCellId::new(576460752840294401),
            ]
        );
    }

    #[test]
    fn various_neighbors() {
        let id = WMCellId::from_zoom_xy(0, 0, 0);
        let neighbors = id.neighbors(false);
        assert_eq!(neighbors.len(), 0);
        assert_eq!(neighbors, Vec::<WMCellId>::new());

        let id = WMCellId::from_zoom_xy(1, 0, 0);
        let neighbors = id.neighbors(false);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors, [WMCellId::from_zoom_xy(1, 0, 1), WMCellId::from_zoom_xy(1, 1, 0)]);

        let id = WMCellId::from_zoom_xy(1, 1, 1);
        let neighbors = id.neighbors(false);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors, [WMCellId::from_zoom_xy(1, 0, 1), WMCellId::from_zoom_xy(1, 1, 0)]);

        let id = WMCellId::from_zoom_xy(2, 0, 0);
        let neighbors = id.neighbors(false);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors, [WMCellId::from_zoom_xy(2, 0, 1), WMCellId::from_zoom_xy(2, 1, 0)]);

        let id = WMCellId::from_zoom_xy(2, 1, 1);
        let neighbors = id.neighbors(false);
        assert_eq!(neighbors.len(), 4);
        assert_eq!(
            neighbors,
            [
                WMCellId::from_zoom_xy(2, 0, 1),
                WMCellId::from_zoom_xy(2, 1, 0),
                WMCellId::from_zoom_xy(2, 1, 2),
                WMCellId::from_zoom_xy(2, 2, 1),
            ]
        );

        let id = WMCellId::from_zoom_xy(0, 0, 0);
        let neighbors = id.neighbors(true);
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn tile_id_wrapped() {
        assert_eq!(
            WMCellId::from_zoom_xy(0, 0, 0).tile_id_wrapped(),
            WMCellId::from_zoom_xy(0, 0, 0)
        );

        assert_eq!(
            WMCellId::from_zoom_xy(0, 1, 0).tile_id_wrapped(),
            WMCellId::from_zoom_xy(0, 0, 0)
        );

        assert_eq!(
            WMCellId::from_zoom_xy(0, 2, 0).tile_id_wrapped(),
            WMCellId::from_zoom_xy(0, 0, 0)
        );

        assert_eq!(
            WMCellId::from_zoom_xy(1, 1, 0).tile_id_wrapped(),
            WMCellId::from_zoom_xy(1, 1, 0)
        );

        assert_eq!(
            WMCellId::from_zoom_xy(2, 10, 0).tile_id_wrapped(),
            WMCellId::from_zoom_xy(2, 2, 0)
        );
    }

    #[test]
    fn is_out_of_bounds() {
        assert!(!WMCellId::from_zoom_xy(0, 0, 0).is_out_of_bounds());
        assert!(WMCellId::from_zoom_xy(0, 1, 0).is_out_of_bounds());
        assert!(WMCellId::from_zoom_xy(0, 2, 0).is_out_of_bounds());

        assert!(!WMCellId::from_zoom_xy(1, 0, 0).is_out_of_bounds());
        assert!(!WMCellId::from_zoom_xy(1, 1, 0).is_out_of_bounds());
        assert!(WMCellId::from_zoom_xy(1, 2, 0).is_out_of_bounds());

        assert!(!WMCellId::from_zoom_xy(2, 0, 0).is_out_of_bounds());
        assert!(!WMCellId::from_zoom_xy(2, 1, 0).is_out_of_bounds());
        assert!(!WMCellId::from_zoom_xy(2, 2, 0).is_out_of_bounds());
        assert!(!WMCellId::from_zoom_xy(2, 3, 0).is_out_of_bounds());
        assert!(WMCellId::from_zoom_xy(2, 4, 0).is_out_of_bounds());

        assert!(WMCellId::from_zoom_xy(2, 10, 0).is_out_of_bounds());
    }

    #[test]
    fn parent() {
        assert_eq!(WMCellId::from_zoom_xy(1, 0, 0).parent(), WMCellId::from_zoom_xy(0, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(2, 0, 0).parent(), WMCellId::from_zoom_xy(1, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(2, 2, 1).parent(), WMCellId::from_zoom_xy(1, 1, 0));
        assert_eq!(WMCellId::from_zoom_xy(22, 0, 0).parent(), WMCellId::from_zoom_xy(21, 0, 0));
    }

    #[test]
    #[should_panic(expected = "You can not find a parent at the face level.")]
    fn parent_panic() {
        let _ = WMCellId::from_zoom_xy(0, 0, 0).parent();
    }

    #[test]
    fn to_ij() {
        assert_eq!(WMCellId::from_zoom_xy(0, 0, 0).to_zoom_ij(None), (0, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(2, 0, 0).to_zoom_ij(None), (2, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(2, 0, 0).to_zoom_ij(Some(1)), (1, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(2, 0, 0).to_zoom_ij(Some(0)), (0, 0, 0));

        // zoom 10
        assert_eq!(WMCellId::from_zoom_xy(10, 20, 20).to_zoom_ij(None), (10, 20, 20));
        assert_eq!(WMCellId::from_zoom_xy(10, 20, 20).to_zoom_ij(Some(9)), (9, 10, 10));
        assert_eq!(WMCellId::from_zoom_xy(10, 20, 20).to_zoom_ij(Some(8)), (8, 5, 5));
        assert_eq!(WMCellId::from_zoom_xy(10, 20, 20).to_zoom_ij(Some(7)), (7, 2, 2));
        assert_eq!(WMCellId::from_zoom_xy(10, 20, 20).to_zoom_ij(Some(5)), (5, 0, 0));
        assert_eq!(WMCellId::from_zoom_xy(10, 20, 20).to_zoom_ij(Some(1)), (1, 0, 0));
    }

    #[test]
    fn contains() {
        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(1, 0, 0)));
        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(1, 1, 0)));
        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(1, 0, 1)));
        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(1, 1, 1)));

        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(5, 0, 0)));
        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(5, 1, 0)));
        assert!(WMCellId::from_zoom_xy(0, 0, 0).contains(&WMCellId::from_zoom_xy(10, 100, 100)));

        assert!(WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(3, 0, 0)));
        assert!(WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(3, 1, 0)));
        assert!(WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(3, 0, 1)));
        assert!(!WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(3, 4, 1)));

        assert!(WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(5, 0, 0)));
        assert!(!WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(5, 16, 0)));
        assert!(!WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(10, 500, 100)));

        assert!(!WMCellId::from_zoom_xy(2, 0, 0).contains(&WMCellId::from_zoom_xy(0, 0, 0)));
    }

    #[test]
    fn is_face() {
        assert!(WMCellId::from_zoom_xy(0, 0, 0).is_face());
        assert!(!WMCellId::from_zoom_xy(1, 0, 0).is_face());
        assert!(!WMCellId::from_zoom_xy(2, 0, 0).is_face());
        assert!(!WMCellId::from_zoom_xy(20, 0, 0).is_face());
    }

    #[test]
    fn level() {
        assert_eq!(WMCellId::from_zoom_xy(0, 0, 0).level(), 0);
        assert_eq!(WMCellId::from_zoom_xy(1, 0, 0).level(), 1);
        assert_eq!(WMCellId::from_zoom_xy(2, 0, 0).level(), 2);
        assert_eq!(WMCellId::from_zoom_xy(20, 0, 0).level(), 20);

        assert_eq!(WMCellId::from_zoom_xy(10, 0, 0).level(), 10);
        assert_eq!(WMCellId::from_zoom_xy(10, 500, 100).level(), 10);
        assert_eq!(WMCellId::from_zoom_xy(29, (1 << 29) - 1, (1 << 29) - 1).level(), 29);
    }
}
