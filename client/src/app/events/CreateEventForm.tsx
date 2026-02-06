import { useState } from 'react';
import { eventsApi } from '@/lib/api';
import { CreateEventRequest, Location } from '@/lib/types';

interface CreateEventFormProps {
  onSuccess: () => void;
  onCancel: () => void;
}

const CreateEventForm = ({ onSuccess, onCancel }: CreateEventFormProps) => {
  const [formData, setFormData] = useState<CreateEventRequest>({
    title: '',
    description: '',
    date: new Date().toISOString().split('T')[0],
    time: '09:00:00',
    location: 'DOA' as Location,
    attendance_type: 'Mandatory',
    grace_period_in_minutes: 15,
  });
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      await eventsApi.create(formData);
      onSuccess();
    } catch (err) {
      alert('Failed to create event');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Title</label>
        <input
          required
          type="text"
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={formData.title}
          onChange={(e) => setFormData({ ...formData, title: e.target.value })}
        />
      </div>
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Description</label>
        <textarea
          required
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={formData.description}
          onChange={(e) => setFormData({ ...formData, description: e.target.value })}
        />
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Date</label>
          <input
            required
            type="date"
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.date}
            onChange={(e) => setFormData({ ...formData, date: e.target.value })}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Time</label>
          <input
            required
            type="time"
            step="1"
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.time}
            onChange={(e) => setFormData({ ...formData, time: e.target.value })}
          />
        </div>
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Location</label>
          <select
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.location}
            onChange={(e) => setFormData({ ...formData, location: e.target.value as Location })}
          >
            <option value="DOA">DOA</option>
            <option value="CHIDA">CHIDA</option>
            <option value="OTHER">OTHER</option>
          </select>
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Attendance Type</label>
          <select
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.attendance_type}
            onChange={(e) => setFormData({ ...formData, attendance_type: e.target.value })}
          >
            <option value="Mandatory">Mandatory</option>
            <option value="Optional">Optional</option>
            <option value="Onsite">Onsite</option>
            <option value="Remote">Remote</option>
          </select>
        </div>
      </div>
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Grace Period (minutes)</label>
        <input
          type="number"
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={formData.grace_period_in_minutes}
          onChange={(e) => setFormData({ ...formData, grace_period_in_minutes: parseInt(e.target.value) })}
        />
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
          className="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? 'Creating...' : 'Create Event'}
        </button>
      </div>
    </form>
  );
};

export default CreateEventForm;
