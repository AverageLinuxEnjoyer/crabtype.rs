#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructureState {
    pub letters_per_row: usize,
    pub rows: usize,
    pub max_written_rows: usize,
}
