pub use coco_branch::CocoBranch;
pub use coco_commit::CocoCommit;

pub mod coco_branch;
pub mod coco_commit;

pub struct GitFileChange {
    pub added: i32,
    pub deleted: i32,
    pub file: String,
    pub mode: String,
}
