'use client';

import React from 'react';
import useSWR from 'swr';
import DeviceMap from '@/components/map/DeviceMap';
import EnergyChart from '@/components/charts/EnergyChart';
import KPIGrid from '@/components/stats/KPIGrid';
import { getLatestDeviceData, getAggregatedHistory, DeviceData, AggregatedData } from '@/app/actions';

interface DashboardProps {
    initialDevices: DeviceData[];
    initialHistory: AggregatedData[];
}

// Fetcher wrappers for SWR
const fetchDevices = () => getLatestDeviceData();
const fetchHistory = () => getAggregatedHistory();

export default function Dashboard({ initialDevices, initialHistory }: DashboardProps) {
    // Poll every 2 seconds for devices, 5 seconds for history
    const { data: devices } = useSWR('devices', fetchDevices, {
        fallbackData: initialDevices,
        refreshInterval: 2000
    });

    const { data: history } = useSWR('history', fetchHistory, {
        fallbackData: initialHistory,
        refreshInterval: 5000
    });

    return (
        <main className="min-h-screen bg-background text-foreground p-4 md:p-6 font-sans">
            <header className="mb-8 flex justify-between items-center">
                <div>
                    <h1 className="text-3xl font-bold text-accent tracking-tighter">
                        NordicPulse
                    </h1>
                    <p className="text-muted text-sm mt-1 uppercase tracking-widest opacity-80">Real-time Energy Grid Simulation</p>
                </div>
                <div className="flex gap-4 items-center">
                    <div className="flex items-center gap-2">
                        <span className="w-2 h-2 rounded-full bg-accent animate-pulse"></span>
                        <span className="text-xs font-mono text-accent">LIVE</span>
                    </div>
                    <div className="text-right text-xs text-muted">
                        Lat: 60.1699 | Lon: 24.9384
                    </div>
                </div>
            </header>

            {/* KPI Stats */}
            <KPIGrid devices={devices || []} />

            {/* Main Grid */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 h-[calc(100vh-220px)] min-h-[600px]">
                {/* Map Section (2/3 width) */}
                <div className="lg:col-span-2 relative h-full">
                    <DeviceMap devices={devices || []} />
                </div>

                {/* Charts Section (1/3 width) */}
                <div className="h-full flex flex-col gap-6">
                    <EnergyChart data={history || []} />

                    {/* Additional mini-panels could go here */}
                    <div className="bg-background border border-muted/20 p-4 rounded-xl flex-1 backdrop-blur">
                        <h3 className="text-muted text-xs font-bold uppercase mb-4 tracking-widest">System Events</h3>
                        <div className="space-y-3 text-sm text-foreground/80 font-mono">
                            <div className="flex justify-between border-b border-muted/10 pb-2">
                                <span>Ingestor Status</span>
                                <span className="text-accent">ONLINE</span>
                            </div>
                            <div className="flex justify-between border-b border-muted/10 pb-2">
                                <span>Simulator Status</span>
                                <span className="text-accent">RUNNING</span>
                            </div>
                            <div className="flex justify-between border-b border-muted/10 pb-2">
                                <span>Database</span>
                                <span className="text-accent">CONNECTED</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    );
}
