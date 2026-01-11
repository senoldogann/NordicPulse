'use server';

import pool from '@/lib/db';

export interface DeviceData {
    device_id: string;
    device_type: string;
    value: number;
    unit: string;
    location: string;
    timestamp: Date;
}

export interface AggregatedData {
    bucket: string;
    device_type: string;
    avg_val: number;
}

/**
 * Fetches the latest data point for each device.
 * Used for the Map visualization.
 */
export async function getLatestDeviceData(): Promise<DeviceData[]> {
    const client = await pool.connect();
    try {
        // DISTINCT ON (device_id) needs ORDER BY device_id, time DESC
        const query = `
      SELECT DISTINCT ON (device_id)
        device_id,
        device_type,
        value,
        unit,
        location,
        time as timestamp
      FROM iot_data
      ORDER BY device_id, time DESC;
    `;
        const res = await client.query(query);
        return res.rows;
    } finally {
        client.release();
    }
}

/**
 * Fetches aggregated average values per minute for the last hour.
 * Used for the Charts.
 */
export async function getAggregatedHistory(): Promise<AggregatedData[]> {
    const client = await pool.connect();
    try {
        const query = `
      SELECT 
        time_bucket('1 minute', time) AS bucket,
        device_type,
        AVG(value) as avg_val
      FROM iot_data
      WHERE time > NOW() - INTERVAL '1 hour'
      GROUP BY bucket, device_type
      ORDER BY bucket ASC;
    `;
        const res = await client.query(query);

        // Convert Dates to ISO strings for serialization boundary if needed, 
        // but Next.js Server Actions can handle Dates usually. 
        // However, recommeding converting to string to be safe for Recharts.
        return res.rows.map(row => ({
            ...row,
            bucket: row.bucket.toISOString(),
            avg_val: parseFloat(row.avg_val) // AVG returns string in pg
        }));
    } finally {
        client.release();
    }
}
