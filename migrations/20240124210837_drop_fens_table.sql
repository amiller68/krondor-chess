-- We don't need the current_fen_id column anymore for games
ALTER TABLE games DROP COLUMN current_fen_id;
-- We don't need the fen_id column anymore for moves
ALTER TABLE moves DROP COLUMN fen_id;

-- We dont need the fens table anymore
DROP TABLE fens;