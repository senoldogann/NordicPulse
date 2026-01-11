'use client';

import React, { useCallback, useMemo } from 'react';
import Map, { NavigationControl, useControl } from 'react-map-gl/maplibre';
import { MapboxOverlay } from '@deck.gl/mapbox';
import { ScatterplotLayer } from '@deck.gl/layers';
import 'maplibre-gl/dist/maplibre-gl.css';
import { DeviceData } from '@/app/actions';

interface DeviceMapProps {
    devices: DeviceData[];
}

const INITIAL_VIEW_STATE = {
    longitude: 25.7482,
    latitude: 63.8266, // Center of Finland
    zoom: 5,
    pitch: 0,
    bearing: 0
};

// DeckGL overlay component using MapboxOverlay for proper sync
function DeckGLOverlay(props: { layers: any[] }) {
    const overlay = useControl<MapboxOverlay>(() => new MapboxOverlay({ interleaved: true }));
    overlay.setProps(props);
    return null;
}

export default function DeviceMap({ devices }: DeviceMapProps) {
    const layers = useMemo(() => [
        new ScatterplotLayer<DeviceData>({
            id: 'device-layer',
            data: devices,
            pickable: true,
            opacity: 0.8,
            stroked: true,
            filled: true,
            radiusScale: 100,
            radiusMinPixels: 3,
            radiusMaxPixels: 20,
            lineWidthMinPixels: 1,
            getPosition: (d: DeviceData) => {
                const [lat, lon] = d.location.split(',').map(Number);
                return [lon, lat];
            },
            getFillColor: (d: DeviceData) => {
                if (d.device_type === 'SolarPanel') return [217, 119, 87]; // #d97757
                if (d.device_type === 'EVCharger') return [240, 238, 230]; // #f0eee6
                return [143, 142, 137]; // #8f8e89
            },
            getLineColor: [20, 20, 19], // #141413
        })
    ], [devices]);

    return (
        <div className="relative w-full h-full rounded-xl overflow-hidden shadow-2xl border border-slate-700/50">
            <Map
                initialViewState={INITIAL_VIEW_STATE}
                mapStyle="https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json"
                style={{ width: '100%', height: '100%' }}
            >
                <DeckGLOverlay layers={layers} />
                <NavigationControl position="top-left" />
            </Map>
        </div>
    );
}
