-- ALTER THE MOVES TABLE

-- Drop Unique constraint on moves (game_id, move_number)
ALTER TABLE moves DROP CONSTRAINT moves_game_id_move_number_key;

-- Drop our index on moves (game_id)
DROP INDEX idx_moves_game_id;

-- Drop the game_id, move, and move_number columns
ALTER TABLE moves DROP COLUMN game_id;
ALTER TABLE moves DROP COLUMN move_number;
ALTER TABLE moves DROP COLUMN move;

-- Insert just a plain fen column that should be unique
ALTER TABLE moves ADD COLUMN board TEXT UNIQUE NOT NULL;

-- RENAME THE MOVES TABLE TO POSITIONS

-- Rename the entire moves table to positions
--  This will describe every unqiue position that a game has been in 
ALTER TABLE moves RENAME TO positions;

-- CREATE A NEW MOVES TABLE

-- Create a new table, also called moves, to track moves made in a game
CREATE TABLE IF NOT EXISTS moves (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    -- The game id
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    -- The position of the game and the last move made
    position_id UUID NOT NULL REFERENCES positions(id) ON DELETE CASCADE,
    -- TODO: do we need this? This information is available within the FEN of the position
    -- The move number of the game
    move_number INTEGER NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- enforce uniqueness on point in a game. also implement an index
    UNIQUE (game_id, move_number)
);