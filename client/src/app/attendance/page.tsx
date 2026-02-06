'use client';

import { useState, useEffect } from 'react';
import { attendanceApi } from '@/lib/api';
import { MapPin, CheckCircle, AlertCircle, Loader2 } from 'lucide-react';

const AttendancePage = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [location, setLocation] = useState<{ lat: number; lng: number } | null>(null);

  useEffect(() => {
    if ('geolocation' in navigator) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setLocation({
            lat: position.coords.latitude,
            lng: position.coords.longitude,
          });
        },
        (err) => {
          console.error('Error getting location:', err);
          setError('Location access denied. Please enable location to sign attendance.');
        }
      );
    } else {
      setError('Geolocation is not supported by your browser.');
    }
  }, []);

  const handleSignAttendance = async () => {
    if (!location) {
      setError('Waiting for location...');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      // In a real app, device_id would be a persistent unique identifier
      // For this demo, we'll use a simple fallback or a placeholder
      const deviceId = 'web-browser-client';

      const response = await attendanceApi.sign({
        location: { lat: location.lat, lng: location.lng },
        device_id: deviceId,
      });

      setSuccess(response.data.message || 'Attendance signed successfully!');
    } catch (err: any) {
      console.error('Failed to sign attendance:', err);
      setError(err.response?.data?.message || 'Failed to sign attendance. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-[60vh] flex-col items-center justify-center py-12">
      <div className="w-full max-w-md rounded-2xl border border-gray-200 bg-white p-8 shadow-xl dark:border-gray-700 dark:bg-gray-800">
        <div className="mb-6 text-center">
          <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400">
            <MapPin className="h-8 w-8" />
          </div>
          <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Sign Attendance</h1>
          <p className="mt-2 text-gray-500 dark:text-gray-400"> Register your presence for today's session</p>
        </div>

        {error && (
          <div className="mb-6 flex items-center rounded-lg bg-red-50 p-4 text-red-700 dark:bg-red-900/20 dark:text-red-400">
            <AlertCircle className="mr-3 h-5 w-5 shrink-0" />
            <p className="text-sm font-medium">{error}</p>
          </div>
        )}

        {success && (
          <div className="mb-6 flex items-center rounded-lg bg-green-50 p-4 text-green-700 dark:bg-green-900/20 dark:text-green-400">
            <CheckCircle className="mr-3 h-5 w-5 shrink-0" />
            <p className="text-sm font-medium">{success}</p>
          </div>
        )}

        <div className="space-y-6">
          <div className="rounded-lg bg-gray-50 p-4 dark:bg-gray-700/50">
            <h3 className="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">Your Location</h3>
            {location ? (
              <p className="font-mono text-sm text-gray-700 dark:text-gray-300">
                Lat: {location.lat.toFixed(6)} <br />
                Lng: {location.lng.toFixed(6)}
              </p>
            ) : (
              <div className="flex items-center text-sm text-gray-500 dark:text-gray-400">
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Retrieving location coordinates...
              </div>
            )}
          </div>

          <button
            onClick={handleSignAttendance}
            disabled={loading || !location}
            className="flex w-full items-center justify-center rounded-xl bg-blue-600 py-4 text-lg font-bold text-white transition-all hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-blue-500/30"
          >
            {loading ? (
              <>
                <Loader2 className="mr-2 h-5 w-5 animate-spin" />
                Signing...
              </>
            ) : (
              'Sign Attendance Now'
            )}
          </button>
        </div>

        <p className="mt-8 text-center text-xs text-gray-400 dark:text-gray-500">
          Attendance can only be signed within proximity of the designated location.
        </p>
      </div>
    </div>
  );
};

export default AttendancePage;
