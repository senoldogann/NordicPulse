'use client';

import { DeviceData } from '@/app/actions';
import { Zap, Server, Activity, Thermometer } from 'lucide-react';

interface KPIGridProps {
    devices: DeviceData[];
}

export default function KPIGrid({ devices }: KPIGridProps) {
    const totalPower = devices.reduce((acc, curr) => acc + curr.value, 0);
    const activeDevices = devices.length;

    return (
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
            <div className="bg-background backdrop-blur border border-muted/30 p-4 rounded-xl">
                <div className="flex items-center gap-3">
                    <div className="p-3 bg-accent/10 rounded-lg text-accent">
                        <Server size={24} />
                    </div>
                    <div>
                        <p className="text-muted text-xs uppercase font-bold tracking-wider">Total Devices</p>
                        <p className="text-2xl font-bold text-foreground">{devices.length}</p>
                    </div>
                </div>
            </div>

            <div className="bg-background backdrop-blur border border-muted/30 p-4 rounded-xl">
                <div className="flex items-center gap-3">
                    <div className="p-3 bg-accent/10 rounded-lg text-accent">
                        <Zap size={24} />
                    </div>
                    <div>
                        <p className="text-muted text-xs uppercase font-bold tracking-wider">Total Active Power</p>
                        <p className="text-2xl font-bold text-foreground">{(totalPower).toFixed(1)} <span className="text-base text-muted">kW</span></p>
                    </div>
                </div>
            </div>

            <div className="bg-background backdrop-blur border border-muted/30 p-4 rounded-xl">
                <div className="flex items-center gap-3">
                    <div className="p-3 bg-accent/10 rounded-lg text-accent">
                        <Activity size={24} />
                    </div>
                    <div>
                        <p className="text-muted text-xs uppercase font-bold tracking-wider">Active Devices</p>
                        <p className="text-2xl font-bold text-foreground">{activeDevices}</p>
                    </div>
                </div>
            </div>

            <div className="bg-background backdrop-blur border border-muted/30 p-4 rounded-xl">
                <div className="flex items-center gap-3">
                    <div className="p-3 bg-accent/10 rounded-lg text-accent">
                        <Thermometer size={24} />
                    </div>
                    <div>
                        <p className="text-muted text-xs uppercase font-bold tracking-wider">System Health</p>
                        <p className="text-2xl font-bold text-foreground">100%</p>
                    </div>
                </div>
            </div>
        </div>
    );
}
