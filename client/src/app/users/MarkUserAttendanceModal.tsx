import { useState } from 'react';
import { attendanceApi } from '@/lib/api';
import { Loader2, CheckCircle2 } from 'lucide-react';
import Modal from '@/components/ui/Modal';

interface MarkUserAttendanceModalProps {
  isOpen: boolean;
  onClose: () => void;
  userId: string;
  userName: string;
}

const MarkUserAttendanceModal = ({ isOpen, onClose, userId, userName }: MarkUserAttendanceModalProps) => {
  const [submitting, setSubmitting] = useState(false);
  const [attendanceType, setAttendanceType] = useState('Onsite');

  const handleMarkAttendance = async () => {
    try {
      setSubmitting(true);
      // @ts-expect-error iii
      await attendanceApi.adminSign(userId, attendanceType);
      alert(`Attendance marked for ${userName}`);
      onClose();
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (err: any) {
      alert(err.response?.data?.message || 'Failed to mark attendance');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={`Mark Weekly Attendance: ${userName}`}>
      <div className="space-y-6">
        <p className="text-sm text-gray-600 dark:text-gray-400">
          This will mark the user as present for today &apos general weekly meeting.
        </p>
        
        <div>
          <label className="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300">Attendance Type</label>
          <select
            className="w-full rounded-lg border border-gray-300 p-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={attendanceType}
            onChange={(e) => setAttendanceType(e.target.value)}
          >
            <option value="Onsite">Onsite</option>
            <option value="Remote">Remote</option>
            <option value="Excused">Excused</option>
            <option value="Late">Late</option>
          </select>
        </div>

        <div className="flex gap-3">
          <button
            onClick={onClose}
            className="flex-1 rounded-lg border border-gray-300 bg-white py-2.5 text-sm font-semibold text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
          >
            Cancel
          </button>
          <button
            onClick={handleMarkAttendance}
            disabled={submitting}
            className="flex flex-1 items-center justify-center rounded-lg bg-green-600 py-2.5 text-sm font-semibold text-white hover:bg-green-700 disabled:opacity-50"
          >
            {submitting ? (
              <Loader2 className="me-2 h-4 w-4 animate-spin" />
            ) : (
              <CheckCircle2 className="me-2 h-4 w-4" />
            )}
            Mark as Present
          </button>
        </div>
      </div>
    </Modal>
  );
};

export default MarkUserAttendanceModal;
