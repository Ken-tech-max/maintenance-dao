#[error]
pub enum MaintenanceError {
    #[msg("")]
    Maintenance,
}

#[error]
pub enum PartConflictError {
    #[msg("Part type already set.")]
    PartConflict,
}

// error for each context
// be as precise as possible