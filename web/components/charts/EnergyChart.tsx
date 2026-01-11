'use client';

import {
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer
} from 'recharts';
import { AggregatedData } from '@/app/actions';

interface EnergyChartProps {
    data: AggregatedData[];
}

export default function EnergyChart({ data }: EnergyChartProps) {
    // Transform data for Recharts: group by timestamp
    // data: [{bucket, device_type, avg_val}, ...] -> [{bucket, SolarPanel, EVCharger, ...}, ...]

    const chartDataMap = new Map<string, any>();

    data.forEach((item) => {
        if (!chartDataMap.has(item.bucket)) {
            chartDataMap.set(item.bucket, { bucket: new Date(item.bucket).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) });
        }
        const entry = chartDataMap.get(item.bucket);
        entry[item.device_type] = item.avg_val;
    });

    const chartData = Array.from(chartDataMap.values());

    return (
        <div className="w-full h-[300px] bg-background p-4 rounded-xl border border-muted/20">
            <h3 className="text-muted font-semibold mb-4 text-sm uppercase tracking-wider">Real-time Energy Flows (Last 1 Hour)</h3>
            <div className="h-64 w-full">
                <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={chartData} margin={{ top: 5, right: 5, left: 10, bottom: 20 }}>
                        <CartesianGrid strokeDasharray="3 3" stroke="#8f8e89" strokeOpacity={0.1} vertical={false} />
                        <XAxis
                            dataKey="bucket"
                            stroke="#8f8e89"
                            fontSize={11}
                            tick={{ fill: '#8f8e89' }}
                            tickMargin={10}
                            axisLine={{ strokeOpacity: 0.3 }}
                            tickLine={false}
                        />
                        <YAxis
                            stroke="#8f8e89"
                            fontSize={11}
                            tick={{ fill: '#8f8e89' }}
                            tickMargin={10}
                            axisLine={{ strokeOpacity: 0.3 }}
                            tickLine={false}
                        />
                        <Tooltip
                            contentStyle={{ backgroundColor: '#141413', borderColor: '#8f8e89', color: '#f0eee6', borderRadius: '8px', border: '1px solid rgba(143, 142, 137, 0.2)' }}
                            itemStyle={{ color: '#f0eee6', fontSize: '12px' }}
                            cursor={{ stroke: '#8f8e89', strokeWidth: 1, strokeDasharray: '5 5' }}
                        />
                        <Legend
                            verticalAlign="bottom"
                            height={36}
                            wrapperStyle={{ paddingTop: '20px', fontSize: '12px', color: '#f0eee6' }}
                        />
                        <Line type="monotone" dataKey="SolarPanel" stroke="#d97757" dot={false} strokeWidth={3} animationDuration={1000} />
                        <Line type="monotone" dataKey="EVCharger" stroke="#f0eee6" dot={false} strokeWidth={2} animationDuration={1000} />
                        <Line type="monotone" dataKey="Sauna" stroke="#8f8e89" dot={false} strokeWidth={2} animationDuration={1000} />
                        <Line type="monotone" dataKey="HeatPump" stroke="#8f8e89" strokeDasharray="5 5" dot={false} strokeWidth={2} animationDuration={1000} />
                    </LineChart>
                </ResponsiveContainer>
            </div>
        </div>
    );
}
