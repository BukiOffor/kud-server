'use client';

import { useState, useEffect } from 'react';
import { analyticsApi } from '@/lib/api';
import { UserAttendanceHistory, UserDto } from '@/lib/types';
import { 
  Loader2, Calendar, CheckSquare, Activity, 
  User as UserIcon, Mail, Shield, Clock, MapPin 
} from 'lucide-react';
import Modal from '@/components/ui/Modal';

interface UserDetailsModalProps {
  userId: string;
  isOpen: boolean;
  onClose: () => void;
}

const UserDetailsModal = ({ userId, isOpen, onClose }: UserDetailsModalProps) => {
  const [loading, setLoading] = useState(true);
  const [data, setData] = useState<UserAttendanceHistory | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen && userId) {
      fetchUserDetails();
    }
  }, [isOpen, userId]);

  const fetchUserDetails = async () => {
    try {
      setLoading(true);
      const response = await analyticsApi.getUserAttendance(userId);
      setData(response.data.data);
      setError(null);
    } catch (err) {
      console.error('Failed to fetch user details:', err);
      setError('Failed to load user attendance history.');
    } finally {
      setLoading(false);
    }
  };

  if (!isOpen) return null;

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="User Profile & History" size="xl">
      <div className="min-h-[400px]">
        {loading ? (
          <div className="flex h-[400px] items-center justify-center">
            <Loader2 className="h-8 w-8 animate-spin text-blue-600" />
          </div>
        ) : error ? (
          <div className="rounded-lg bg-red-50 p-4 text-center text-red-700 dark:bg-red-900/20 dark:text-red-400">
            {error}
          </div>
        ) : data ? (
          <div className="space-y-8">
            {/* Header Info */}
            <div className="flex flex-col gap-6 sm:flex-row sm:items-center sm:justify-between">
              <div className="flex items-center">
                <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400">
                  <UserIcon className="h-8 w-8" />
                </div>
                <div className="ml-4">
                  <h3 className="text-2xl font-bold text-gray-900 dark:text-white">
                    {data.user.first_name} {data.user.last_name}
                  </h3>
                  <div className="flex flex-wrap gap-x-4 gap-y-1 text-sm text-gray-500">
                    <span className="flex items-center">
                      <Mail className="mr-1 h-3 w-3" />
                      {data.user.email}
                    </span>
                    <span className="flex items-center border-l border-gray-200 pl-4 dark:border-gray-700">
                      <Shield className="mr-1 h-3 w-3" />
                      {data.user.role}
                    </span>
                  </div>
                </div>
              </div>
              
              <div className="flex gap-2">
                 <span className="rounded-lg bg-green-100 px-3 py-1 text-xs font-semibold text-green-700 dark:bg-green-900/30 dark:text-green-400">
                   Active Member
                 </span>
              </div>
            </div>

            {/* Stats Grid */}
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div className="rounded-2xl border border-gray-100 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
                <div className="mb-2 flex items-center justify-between text-blue-600">
                  <Activity className="h-5 w-5" />
                  <span className="text-xs font-bold uppercase tracking-wider opacity-60">Attendance Rate</span>
                </div>
                <p className="text-3xl font-black text-gray-900 dark:text-white">
                  {typeof data.summary === 'object' ? data.summary.rate.toFixed(1) : data.summary}%
                </p>
              </div>
              
              <div className="rounded-2xl border border-gray-100 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
                <div className="mb-2 flex items-center justify-between text-green-600">
                  <CheckSquare className="h-5 w-5" />
                  <span className="text-xs font-bold uppercase tracking-wider opacity-60">Days Present</span>
                </div>
                <p className="text-3xl font-black text-gray-900 dark:text-white">
                   {typeof data.summary === 'object' ? data.summary.days_present : data.summary}
                </p>
              </div>

              <div className="rounded-2xl border border-gray-100 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
                <div className="mb-2 flex items-center justify-between text-purple-600">
                  <Calendar className="h-5 w-5" />
                  <span className="text-xs font-bold uppercase tracking-wider opacity-60">Total Tracking</span>
                </div>
                <p className="text-3xl font-black text-gray-900 dark:text-white">
                   {typeof data.summary === 'object' ? data.summary.total_days : data.summary}
                </p>
              </div>
            </div>

            {/* History Table */}
            <div>
              <h4 className="mb-4 text-lg font-bold text-gray-800 dark:text-white">Attendance History</h4>
              <div className="max-h-[300px] overflow-y-auto rounded-xl border border-gray-200 dark:border-gray-700">
                <table className="w-full text-left text-sm text-gray-500 dark:text-gray-400">
                  <thead className="sticky top-0 bg-gray-50 text-xs uppercase text-gray-700 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                      <th className="px-6 py-3">Date</th>
                      <th className="px-6 py-3">Time In</th>
                      <th className="px-6 py-3">Status</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                    {data.history.length > 0 ? (
                      data.history.map((record) => (
                        <tr key={record.id} className="hover:bg-gray-50 dark:hover:bg-gray-700/50">
                          <td className="px-6 py-4 font-medium text-gray-900 dark:text-white">
                            <div className="flex items-center">
                              <Calendar className="mr-2 h-4 w-4 opacity-40" />
                              {record.date}
                            </div>
                          </td>
                          <td className="px-6 py-4">
                            <div className="flex items-center">
                              <Clock className="mr-2 h-4 w-4 opacity-40" />
                              {record.time_in}
                            </div>
                          </td>
                          <td className="px-6 py-4">
                            <span className="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700 dark:bg-green-900/30 dark:text-green-400">
                              Present
                            </span>
                          </td>
                        </tr>
                      ))
                    ) : (
                      <tr>
                        <td colSpan={3} className="px-6 py-10 text-center italic">No attendance records found for this user.</td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        ) : null}
      </div>
    </Modal>
  );
};

export default UserDetailsModal;
