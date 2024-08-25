use alloc::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
        let max_coord = 1 << zoom;
        debug_assert!(
            x < max_coord && y < max_coord,
            "Coordinates must be within bounds for the given zoom level."
        );

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
        let mut neighbors: Vec<Self> = Vec::new();
        let (zoom, x, y) = self.to_zoom_xy();
        let size: u32 = 1 << zoom;
        let x_out_of_bounds = x >= size;

        if x > 0 || include_out_of_bounds {
            neighbors.push(Self::from_zoom_xy(zoom, x - 1, y));
        }
        if x + 1 < size || include_out_of_bounds {
            neighbors.push(Self::from_zoom_xy(zoom, x + 1, y));
        }
        if !x_out_of_bounds && y > 0 {
            neighbors.push(Self::from_zoom_xy(zoom, x, y - 1));
        }
        if !x_out_of_bounds && y + 1 < size {
            neighbors.push(Self::from_zoom_xy(zoom, x, y + 1));
        }

        neighbors
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
        WMCellId::from_zoom_xy(zoom - 1, x >> 1, y >> 1)
    }

    /// convert an id to a zoom-x-y after setting it to a new parent zoom
    pub fn to_zoom_ij(&self, level: Option<u8>) -> (u8, u32, u32) {
        let mut id = *self;
        if let Some(level) = level {
            let (mut curr_zoom, _x, _y) = self.to_zoom_xy();
            while level < curr_zoom {
                id = self.parent();
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
        self.level() != 0
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
