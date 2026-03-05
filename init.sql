-- PixelBoard init script
-- Executed on container startup

\i /docker-entrypoint-initdb.d/migrations/001_create_users/up.sql
\i /docker-entrypoint-initdb.d/migrations/002_create_blocks/up.sql
\i /docker-entrypoint-initdb.d/migrations/003_create_reservations/up.sql
