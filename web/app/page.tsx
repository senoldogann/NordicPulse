import { getLatestDeviceData, getAggregatedHistory } from './actions';
import Dashboard from './Dashboard';

export const dynamic = 'force-dynamic'; // Ensure fresh data on initial load

export default async function Page() {
  // Fetch initial data on server for hydration
  const [initialDevices, initialHistory] = await Promise.all([
    getLatestDeviceData(),
    getAggregatedHistory()
  ]);

  return <Dashboard initialDevices={initialDevices} initialHistory={initialHistory} />;
}
