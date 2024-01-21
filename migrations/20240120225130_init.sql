CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Track all unique FENs
CREATE TABLE IF NOT EXISTS fens (
    id UUID PRIMARY KEY NOT NULL,
    fen TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert the initial FEN if not already present
INSERT INTO fens (id, fen) 
VALUES ('00000000-0000-0000-0000-000000000000', 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR')
ON CONFLICT DO NOTHING;

-- Games are an id and associated metadata
CREATE TABLE IF NOT EXISTS games (
    id uuid PRIMARY KEY NOT NULL,
    -- Current board position -- by default, start position
    current_fen_id UUID REFERENCES fens(id) NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Status of the game
    status VARCHAR(32) NOT NULL DEFAULT 'created',
    CONSTRAINT status_check CHECK (status IN ('created', 'active', 'complete', 'abandoned')),
    -- Winner, if known
    winner VARCHAR(32) DEFAULT NULL,
    CONSTRAINT winner_check CHECK (winner IN ('white', 'black', 'draw')),
    -- Outcome, if known
    outcome VARCHAR(32) DEFAULT NULL,
    CONSTRAINT outcome_check CHECK (outcome IN ('checkmate', 'stalemate', 'resignation'))

);

-- Index games on updated at -- prolly gonna be sorting on this alot
CREATE INDEX IF NOT EXISTS idx_games_updated_at ON games(updated_at);

CREATE TABLE IF NOT EXISTS moves (
    id UUID PRIMARY KEY NOT NULL,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    fen_id UUID NOT NULL REFERENCES fens(id) ON DELETE CASCADE,
    move_number INTEGER NOT NULL,
    move VARCHAR(10) NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- enforce uniqueness on point in a game. also implement an index
    UNIQUE (game_id, move_number)
);

-- Index on Game Id, since we're prolly going to be joining
--  on games and moves quite a bit
CREATE INDEX IF NOT EXISTS idx_moves_game_id ON moves(game_id);
