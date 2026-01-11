import { Pool } from 'pg';

// Use environment variable or fallback for development
const connectionString = process.env.DATABASE_URL || 'postgres://postgres:password@127.0.0.1:5434/nordic_pulse';

const pool = new Pool({
  connectionString,
  max: 10, // Max clients in the pool
  idleTimeoutMillis: 30000,
});

pool.on('error', (err) => {
  console.error('Unexpected error on idle client', err);
  process.exit(-1);
});

export default pool;
