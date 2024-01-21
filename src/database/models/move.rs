#[derive(FromRow)]
pub struct Move {
    id: Uuid,
    game_id: Uuid,
    fen_id: Uuid,
    move_number: i32,
    // TODO: Database tupe for move
    r#move: String,
    created_at: OffsetDateTime
}