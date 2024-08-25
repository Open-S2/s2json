use crate::s2::S2CellId;
use crate::wm::WMCellId;

pub enum CellId {
    S2(S2CellId),
    WM(WMCellId),
}
impl CellId {
    pub fn level(&self) -> u8 {
        match self {
            CellId::S2(id) => id.level(),
            CellId::WM(id) => id.level(),
        }
    }

    pub fn face(&self) -> u8 {
        match self {
            CellId::S2(id) => id.face(),
            CellId::WM(_) => 0,
        }
    }

    pub fn is_face(&self) -> bool {
        match self {
            CellId::S2(id) => id.is_face(),
            CellId::WM(id) => id.is_face(),
        }
    }

    pub fn from_face(face: u8) -> CellId {
        match face {
            0 => CellId::S2(S2CellId::from_face(face)),
            _ => CellId::WM(WMCellId::new(0)),
        }
    }

    pub fn parent(&self) -> CellId {
        match self {
            CellId::S2(id) => CellId::S2(id.parent(None)),
            CellId::WM(id) => CellId::WM(id.parent()),
        }
    }

    pub fn contains(&self, other: &CellId) -> bool {
        match (self, other) {
            (CellId::S2(id1), CellId::S2(id2)) => id1.contains(id2),
            (CellId::WM(id1), CellId::WM(id2)) => id1.contains(id2),
            _ => false,
        }
    }

    /// Given the projection and id, return the face/zoom with the i-j coordinate pair
    /// Returns (face/zoom, i, j)
    pub fn to_zoom_ij(&self, level: Option<u8>) -> (u8, u32, u32) {
        match self {
            CellId::S2(id) => id.to_zoom_ij(level),
            CellId::WM(id) => id.to_zoom_ij(level),
        }
    }

    /// Given the projection, get the children tile IDs
    pub fn children_ij(&self, face: u8, zoom: u8, i: u32, j: u32) -> [CellId; 4] {
        match self {
            CellId::S2(id) => id.children_ij(face, zoom, i, j).map(CellId::S2),
            CellId::WM(id) => id.children().map(CellId::WM),
        }
    }

    /// Decode the CellId back into (face, zoom, x, y) components.
    pub fn to_face_zoom_xy(&self) -> (u8, u8, u32, u32) {
        match self {
            CellId::S2(id) => {
                let zoom = id.level();
                let (face, x, y) = id.to_zoom_ij(None);
                (face, zoom, x, y)
            }
            CellId::WM(id) => {
                let (zoom, x, y) = id.to_zoom_xy();
                (0, zoom, x, y)
            }
        }
    }
}
