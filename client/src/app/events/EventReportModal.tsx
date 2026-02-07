'use client';

import React, { useEffect, useState } from 'react';
import { analyticsApi } from '@/lib/api';
import { EventStatsReport } from '@/lib/types';
import Modal from '@/components/ui/Modal';
import { 
  BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, 
  ResponsiveContainer, AreaChart, Area, PieChart, Pie, Cell, Legend
} from 'recharts';
import { Users, Clock, AlertCircle, Loader2, UserMinus } from 'lucide-react';

interface EventReportModalProps {
  isOpen: boolean;
  onClose: () => void;
  eventId: string;
  eventTitle: string;
}

const COLORS = ['#3b82f6', '#e5e7eb']; // blue for present, gray for absent

const EventReportModal = ({ isOpen, onClose, eventId, eventTitle }: EventReportModalProps) => {
  const [data, setData] = useState<EventStatsReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'attendees' | 'absentees'>('attendees');

  useEffect(() => {
    if (isOpen && eventId) {
      fetchReport();
    }
  }, [isOpen, eventId]);

  const fetchReport = async () => {
    try {
      setLoading(true);
      const response = await analyticsApi.getEventReport(eventId);
      setData(response.data.data);
      setError(null);
    } catch (err) {
      console.error('Failed to fetch event report:', err);
      setError('Failed to load attendance report.');
    } finally {
      setLoading(false);
    }
  };

  // Process data for the timeline graph
  const processTimelineData = () => {
    if (!data || !data.attendees.length) return [];

    const minuteGroups: Record<string, number> = {};
    
    data.attendees.forEach(attendee => {
      const timeStr = attendee.time_in.substring(0, 5);
      minuteGroups[timeStr] = (minuteGroups[timeStr] || 0) + 1;
    });

    const sortedTimes = Object.keys(minuteGroups).sort();
    let cumulative = 0;
    
    return sortedTimes.map(time => {
      cumulative += minuteGroups[time];
      return {
        time,
        count: minuteGroups[time],
        total: cumulative
      };
    });
  };

  const chartData = processTimelineData();
  
  const pieData = data ? [
    { name: 'Present', value: data.total_attendees },
    { name: 'Absent', value: Math.max(0, data.eligible_attendees_count - data.total_attendees) }
  ] : [];

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={`Attendance Report: ${eventTitle}`} size="xl">
      {loading ? (
        <div className="flex h-64 items-center justify-center">
          <Loader2 className="h-8 w-8 animate-spin text-blue-600" />
        </div>
      ) : error ? (
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <AlertCircle className="mb-2 h-12 w-12 text-red-500" />
          <p className="text-gray-800 dark:text-white">{error}</p>
        </div>
      ) : (
        <div className="space-y-8">
          {/* Summary Cards */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
            <div className="flex items-center rounded-xl border border-gray-100 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
              <div className="mr-4 rounded-lg bg-blue-100 p-3 dark:bg-blue-900/30">
                <Users className="h-6 w-6 text-blue-600 dark:text-blue-400" />
              </div>
              <div>
                <p className="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Attendees</p>
                <p className="text-xl font-bold text-gray-900 dark:text-white">{data?.total_attendees} / {data?.eligible_attendees_count}</p>
              </div>
            </div>
            <div className="flex items-center rounded-xl border border-gray-100 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
              <div className="mr-4 rounded-lg bg-orange-100 p-3 dark:bg-orange-900/30">
                <UserMinus className="h-6 w-6 text-orange-600 dark:text-orange-400" />
              </div>
              <div>
                <p className="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Absentees</p>
                <p className="text-xl font-bold text-gray-900 dark:text-white">
                  {data ? Math.max(0, data.eligible_attendees_count - data.total_attendees) : 0}
                </p>
              </div>
            </div>
            <div className="flex items-center rounded-xl border border-gray-100 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
              <div className="mr-4 rounded-lg bg-green-100 p-3 dark:bg-green-900/30">
                <Clock className="h-6 w-6 text-green-600 dark:text-green-400" />
              </div>
              <div>
                <p className="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">First Sign-in</p>
                <p className="text-xl font-bold text-gray-900 dark:text-white">
                  {data?.attendees.length ? data.attendees.sort((a,b) => a.time_in.localeCompare(b.time_in))[0].time_in.substring(0, 5) : '--:--'}
                </p>
              </div>
            </div>
          </div>

          <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
            {/* Timeline Chart */}
            <div className="rounded-xl border border-gray-100 bg-white p-6 dark:border-gray-700 dark:bg-gray-800">
              <h4 className="mb-6 text-sm font-semibold uppercase tracking-wider text-gray-500">Check-in Timeline</h4>
              <div className="h-64 w-full">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={chartData}>
                    <defs>
                      <linearGradient id="colorTotal" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.1}/>
                        <stop offset="95%" stopColor="#3b82f6" stopOpacity={0}/>
                      </linearGradient>
                    </defs>
                    <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="#e5e7eb" />
                    <XAxis 
                      dataKey="time" 
                      stroke="#9ca3af" 
                      fontSize={10} 
                      tickLine={false} 
                      axisLine={false} 
                    />
                    <YAxis 
                      stroke="#9ca3af" 
                      fontSize={10} 
                      tickLine={false} 
                      axisLine={false} 
                    />
                    <Tooltip 
                      contentStyle={{ 
                        backgroundColor: '#1f2937', 
                        border: 'none', 
                        borderRadius: '8px',
                        color: '#fff',
                        fontSize: '12px'
                      }}
                    />
                    <Area 
                      type="monotone" 
                      dataKey="total" 
                      stroke="#3b82f6" 
                      strokeWidth={2}
                      fillOpacity={1} 
                      fill="url(#colorTotal)" 
                      name="Cumulative"
                    />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Attendance Pie Chart */}
            <div className="rounded-xl border border-gray-100 bg-white p-6 dark:border-gray-700 dark:bg-gray-800">
              <h4 className="mb-6 text-sm font-semibold uppercase tracking-wider text-gray-500">Attendance Ratio</h4>
              <div className="h-64 w-full">
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={pieData}
                      cx="50%"
                      cy="50%"
                      innerRadius={60}
                      outerRadius={80}
                      paddingAngle={5}
                      dataKey="value"
                    >
                      {pieData.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                      ))}
                    </Pie>
                    <Tooltip 
                       contentStyle={{ 
                        backgroundColor: '#1f2937', 
                        border: 'none', 
                        borderRadius: '8px',
                        color: '#fff',
                        fontSize: '12px'
                      }}
                    />
                    <Legend verticalAlign="bottom" height={36}/>
                  </PieChart>
                </ResponsiveContainer>
              </div>
            </div>
          </div>

          {/* Attendees / Absentees Tabs */}
          <div className="space-y-4">
            <div className="flex border-b border-gray-100 dark:border-gray-700">
              <button
                onClick={() => setActiveTab('attendees')}
                className={`px-6 py-3 text-sm font-semibold transition-colors ${
                  activeTab === 'attendees'
                    ? 'border-b-2 border-blue-600 text-blue-600'
                    : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'
                }`}
              >
                Attendees ({data?.attendees.length})
              </button>
              <button
                onClick={() => setActiveTab('absentees')}
                className={`px-6 py-3 text-sm font-semibold transition-colors ${
                  activeTab === 'absentees'
                    ? 'border-b-2 border-blue-600 text-blue-600'
                    : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'
                }`}
              >
                Absentees ({data?.absentees.length})
              </button>
            </div>

            <div className="overflow-hidden rounded-xl border border-gray-100 dark:border-gray-700">
              <table className="w-full text-left text-sm text-gray-500 dark:text-gray-400">
                <thead className="bg-gray-50 text-xs font-semibold uppercase text-gray-700 dark:bg-gray-800 dark:text-gray-300">
                  <tr>
                    <th className="px-6 py-4">User</th>
                    <th className="px-6 py-4">Email</th>
                    {activeTab === 'attendees' && <th className="px-6 py-4 text-right">Check-in Time</th>}
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100 dark:divide-gray-700">
                  {activeTab === 'attendees' ? (
                    data?.attendees.map((attendee) => (
                      <tr key={attendee.user_id} className="bg-white hover:bg-gray-50 dark:bg-gray-900 dark:hover:bg-gray-800/50">
                        <td className="px-6 py-4 font-medium text-gray-900 dark:text-white">
                          {attendee.first_name} {attendee.last_name}
                        </td>
                        <td className="px-6 py-4">{attendee.email}</td>
                        <td className="px-6 py-4 text-right font-mono">
                          {attendee.time_in.substring(0, 5)}
                        </td>
                      </tr>
                    ))
                  ) : (
                    data?.absentees.map((absentee) => (
                      <tr key={absentee.id} className="bg-white hover:bg-gray-50 dark:bg-gray-900 dark:hover:bg-gray-800/50">
                        <td className="px-6 py-4 font-medium text-gray-900 dark:text-white">
                          {absentee.first_name} {absentee.last_name}
                        </td>
                        <td className="px-6 py-4">{absentee.email}</td>
                      </tr>
                    ))
                  )}
                  {((activeTab === 'attendees' && data?.attendees.length === 0) || 
                    (activeTab === 'absentees' && data?.absentees.length === 0)) && (
                    <tr>
                      <td colSpan={3} className="px-6 py-12 text-center text-gray-500">
                        No {activeTab} found.
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}
    </Modal>
  );
};

export default EventReportModal;
