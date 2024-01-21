#[derive(FromRow)]
pub struct Fen {
    id: Uuid,
    // TODO: Database type for FEN
    fen: String,
    created_at: OffsetDateTime,
}
