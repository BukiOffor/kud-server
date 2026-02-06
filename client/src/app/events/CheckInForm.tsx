'use client';

import { useState } from 'react';
import { eventsApi } from '@/lib/api';
import { CheckIntoEventRequest } from '@/lib/types';

interface CheckInFormProps {
  eventId: string;
  onSuccess: () => void;
  onCancel: () => void;
}

const CheckInForm = ({ eventId, onSuccess, onCancel }: CheckInFormProps) => {
  const [userId, setUserId] = useState('');
  const [attendanceType, setAttendanceType] = useState('Standard');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      await eventsApi.checkIn({
        event_id: eventId,
        user_id: userId,
        attendance_type: attendanceType,
      });
      onSuccess();
    } catch (err) {
      alert('Failed to check in user. Ensure the User ID is valid.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">User ID (UUID)</label>
        <input
          required
          type="text"
          placeholder="e.g. 550e8400-e29b-41d4-a716-446655440000"
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={userId}
          onChange={(e) => setUserId(e.target.value)}
        />
      </div>
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Attendance Type</label>
        <select
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={attendanceType}
          onChange={(e) => setAttendanceType(e.target.value)}
        >
          <option value="Standard">Standard</option>
          <option value="Late">Late</option>
          <option value="Excused">Excused</option>
        </select>
      </div>
      <div className="flex justify-end gap-2 pt-4">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-lg border px-4 py-2 text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={loading}
          className="rounded-lg bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700 disabled:opacity-50"
        >
          {loading ? 'Checking in...' : 'Check In'}
        </button>
      </div>
    </form>
  );
};

export default CheckInForm;
