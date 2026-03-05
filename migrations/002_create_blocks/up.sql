CREATE TABLE IF NOT EXISTS blocks (
    id           VARCHAR(36)  PRIMARY KEY,
    user_id      VARCHAR(36)  NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    x            INT          NOT NULL,
    y            INT          NOT NULL,
    width        INT          NOT NULL CHECK (width >= 10),
    height       INT          NOT NULL CHECK (height >= 10),
    image_url    VARCHAR(512) NOT NULL,
    link         VARCHAR(512) NOT NULL,
    title        VARCHAR(63)  NOT NULL,
    description  VARCHAR(255),
    status       VARCHAR(20)  NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'reserved', 'expired')),
    price_cents  BIGINT       NOT NULL,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ
);

CREATE INDEX idx_blocks_user_id ON blocks (user_id);
CREATE INDEX idx_blocks_status  ON blocks (status);
CREATE INDEX idx_blocks_coords  ON blocks (x, y, width, height);
