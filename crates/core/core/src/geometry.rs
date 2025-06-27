use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
