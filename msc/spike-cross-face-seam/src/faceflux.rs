//! Face-keyed flux as a real `store.rs` citizen — `form-face-flux-register` +
//! `form-grid-equiangular-staggered` FE(2) ("a face owned by exactly one of its
//! two cells, canonically the lower `CellId`").
//!
//! A *face* is the boundary shared by two adjacent cells. Its identity is the
//! **unordered pair** of their `CellId`s; canonically the lower id owns it. This
//! is well-defined **across cube faces** because `CellId`'s `Ord` puts the face
//! index in the top bits — a global order, so "lower id" needs no per-face
//! tie-break.
//!
//! Two layers, kept honest:
//!  - **hot loop**: an in-memory `HashMap<FaceId, f64>` register. One flux per
//!    face, applied once with opposite signs (`+F` to one cell, `−F` to the
//!    other). Conservation is a property of *applying one value once*, not of
//!    the flux's accuracy (`form-face-flux-register` FE(5)).
//!  - **persistence / census**: each face has a canonical content-addressed
//!    [`Key`] in the real `store.rs`, so the register *is* a store citizen — it
//!    persists, dedups, and enumerates by meaning. (Per-substep accumulation
//!    stays in memory; the store is the identity + save layer, not a per-step
//!    filesystem write. Stated plainly so the "store citizen" claim isn't
//!    overread.)

use std::collections::HashMap;
use vivarium_world::sphere::CellId;
use vivarium_world::store::{Key, Store};

/// Canonical identity of the face between two adjacent cells: the ordered pair
/// `(lo, hi)` with `lo` the owner (lower `CellId`). Symmetric by construction —
/// both cells derive the *same* `FaceId`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FaceId {
    pub lo: CellId,
    pub hi: CellId,
}

impl FaceId {
    /// Build from the two cells in **either** order (the symmetry that a
    /// per-face-local index rule would break across a seam).
    pub fn between(a: CellId, b: CellId) -> Self {
        if a <= b { FaceId { lo: a, hi: b } } else { FaceId { lo: b, hi: a } }
    }

    /// The owning cell — FE(2)'s "lower `CellId`".
    #[inline]
    pub fn owner(self) -> CellId { self.lo }

    /// The complete content-addressed key for this face in the store.
    pub fn key(self) -> Key {
        Key::new("face-flux", "v0")
            .field("lo", self.lo.0)
            .field("hi", self.hi.0)
    }
}

/// The single-valued face-flux register (the hot-loop layer).
#[derive(Default)]
pub struct FluxRegister {
    fluxes: HashMap<FaceId, f64>,
}

impl FluxRegister {
    pub fn new() -> Self { Self::default() }

    /// Set the single flux on a face (sign convention: positive = flow from `lo`
    /// to `hi`). Idempotent — one object per face.
    pub fn set(&mut self, face: FaceId, flux: f64) {
        self.fluxes.insert(face, flux);
    }

    pub fn get(&self, face: FaceId) -> f64 {
        self.fluxes.get(&face).copied().unwrap_or(0.0)
    }

    pub fn len(&self) -> usize { self.fluxes.len() }
    pub fn is_empty(&self) -> bool { self.fluxes.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = (&FaceId, &f64)> { self.fluxes.iter() }

    /// Persist the whole register through the real content-addressed store —
    /// each face under its canonical key. Returns the number of roots written.
    pub fn persist(&self, store: &Store) -> std::io::Result<usize> {
        for (face, &flux) in &self.fluxes {
            store.put(&face.key(), &flux.to_le_bytes())?;
        }
        Ok(self.fluxes.len())
    }

    /// Reload one face's flux from the store (the save→reopen path).
    pub fn load(store: &Store, face: FaceId) -> Option<f64> {
        let bytes = store.get(&face.key())?;
        let arr: [u8; 8] = bytes.try_into().ok()?;
        Some(f64::from_le_bytes(arr))
    }
}
