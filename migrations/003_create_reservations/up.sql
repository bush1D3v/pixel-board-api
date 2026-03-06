CREATE TABLE IF NOT EXISTS reservations (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    block_id VARCHAR(36) REFERENCES blocks (id) ON DELETE SET NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    width INT NOT NULL CHECK (width >= 10),
    height INT NOT NULL CHECK (height >= 10),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (
        status IN (
            'pending',
            'confirmed',
            'expired'
        )
    ),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reservations_user_id ON reservations (user_id);

CREATE INDEX IF NOT EXISTS idx_reservations_status ON reservations (status);

CREATE INDEX IF NOT EXISTS idx_reservations_expires_at ON reservations (expires_at);
