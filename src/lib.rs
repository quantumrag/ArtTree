
mod nodes;
mod art;

use nodes::ArtNode;

pub trait ArtKey {
    fn bytes(&self) -> &[u8];
}

pub struct ArtTree<K: ArtKey, V> {
    root: ArtNode<K, V>,
    size: usize,
}
