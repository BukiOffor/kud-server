'use client';

import { useState, useEffect } from 'react';
import { analyticsApi } from '@/lib/api';
import { AttendanceStats, UserDto } from '@/lib/types';
import { 
  Users, TrendingUp, Calendar, Gift, 
  BarChart, PieChart, Activity, Loader2,
  ChevronRight, Cake, User
} from 'lucide-react';

const AnalyticsPage = () => {
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState<AttendanceStats | null>(null);
  const [upcomingBirthdays, setUpcomingBirthdays] = useState<UserDto[]>([]);
  const [error, setError] = useState<string | null>(null);
  
  // Day-Specific Stats
  const [selectedDate, setSelectedDate] = useState(new Date().toISOString().split('T')[0]);
  const [dayStats, setDayStats] = useState<{ presentees: UserDto[], absentees: UserDto[] } | null>(null);
  const [loadingDayStats, setLoadingDayStats] = useState(false);

  useEffect(() => {
    fetchAnalytics();
    fetchDayStats(selectedDate);
  }, []);

  const fetchAnalytics = async () => {
    try {
      setLoading(true);
      const [ratesRes, birthdaysRes] = await Promise.all([
        analyticsApi.getAttendanceRates(),
        analyticsApi.getUpcomingBirthdays()
      ]);
      
      setStats(ratesRes.data.message);
      setUpcomingBirthdays(birthdaysRes.data.message);
      setError(null);
    } catch (err) {
      console.error('Failed to fetch analytics:', err);
      setError('Failed to load analytics data.');
    } finally {
      setLoading(false);
    }
  };

  const fetchDayStats = async (date: string) => {
    try {
      setLoadingDayStats(true);
      const res = await analyticsApi.getUsersPresentOnDay(date);
      setDayStats({
        presentees: res.data.message.presentees,
        absentees: res.data.message.absentees
      });
    } catch (err) {
      console.error('Failed to fetch day stats:', err);
    } finally {
      setLoadingDayStats(false);
    }
  };

  const handleDateChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newDate = e.target.value;
    setSelectedDate(newDate);
    fetchDayStats(newDate);
  };

  if (loading) {
    return (
      <div className="flex h-[60vh] items-center justify-center">
        <div className="text-center">
          <Loader2 className="mx-auto h-12 w-12 animate-spin text-blue-600" />
          <p className="mt-4 text-gray-500">Loading analytic data...</p>
        </div>
      </div>
    );
  }

  const rateStats = [
    { label: 'Admin Attendance', value: `${(stats?.admin_rate || 0).toFixed(1)}%`, color: 'text-blue-600', bg: 'bg-blue-100' },
    { label: 'User Attendance', value: `${(stats?.user_rate || 0).toFixed(1)}%`, color: 'text-green-600', bg: 'bg-green-100' },
    { label: 'Technical Attendance', value: `${(stats?.technical_rate || 0).toFixed(1)}%`, color: 'text-purple-600', bg: 'bg-purple-100' },
  ];

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold text-gray-800 dark:text-white">Analytics Dashboard</h1>
        <p className="mt-2 text-gray-500 dark:text-gray-400">Detailed insights into attendance and community engagement.</p>
      </div>

      {/* Hero Stats */}
      <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
        <div className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="flex items-center">
            <div className="rounded-xl bg-indigo-100 p-3 text-indigo-600 dark:bg-indigo-900/30 dark:text-indigo-400">
              <Users className="h-6 w-6" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Total Users</p>
              <h3 className="text-2xl font-bold">{stats?.total_users || 0}</h3>
            </div>
          </div>
        </div>
        
        {rateStats.map((item) => (
          <div key={item.label} className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
            <div className="flex items-center">
              <div className={`rounded-xl ${item.bg} p-3 ${item.color} dark:bg-gray-700`}>
                <Activity className="h-6 w-6" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">{item.label}</p>
                <h3 className="text-2xl font-bold">{item.value}</h3>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Attendance on Specific Day */}
      <div className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <h2 className="text-xl font-bold">Daily Attendance Tracker</h2>
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-500 italic">Filter by Date:</span>
            <input 
              type="date" 
              value={selectedDate}
              onChange={handleDateChange}
              className="rounded-lg border border-gray-200 bg-white px-3 py-1 text-sm shadow-sm transition-all focus:border-blue-500 focus:outline-none dark:border-gray-700 dark:bg-gray-800"
            />
          </div>
        </div>
        
        <div className="grid grid-cols-1 gap-8 md:grid-cols-2">
          {/* Presentees */}
          <div>
            <h3 className="mb-4 flex items-center text-sm font-semibold uppercase tracking-wider text-green-600">
              <div className="mr-2 h-2 w-2 rounded-full bg-green-500"></div>
              Present ({dayStats?.presentees.length || 0})
            </h3>
            {loadingDayStats ? (
              <Loader2 className="h-5 w-5 animate-spin text-gray-300" />
            ) : dayStats?.presentees.length ? (
              <div className="space-y-2">
                {dayStats.presentees.map(u => (
                  <div key={u.id} className="flex items-center rounded-lg bg-green-50/50 p-2 text-sm text-green-800 dark:bg-green-900/10 dark:text-green-400">
                    <User className="mr-2 h-4 w-4" />
                    {u.first_name} {u.last_name}
                  </div>
                ))}
              </div>
            ) : <p className="text-sm text-gray-400 italic">No records found.</p>}
          </div>

          {/* Absentees */}
          <div>
            <h3 className="mb-4 flex items-center text-sm font-semibold uppercase tracking-wider text-red-600">
              <div className="mr-2 h-2 w-2 rounded-full bg-red-500"></div>
              Absent ({dayStats?.absentees.length || 0})
            </h3>
            {loadingDayStats ? (
              <Loader2 className="h-5 w-5 animate-spin text-gray-300" />
            ) : dayStats?.absentees.length ? (
              <div className="space-y-2">
                {dayStats.absentees.map(u => (
                  <div key={u.id} className="flex items-center rounded-lg bg-red-50/50 p-2 text-sm text-red-800 dark:bg-red-900/10 dark:text-red-400">
                    <User className="mr-2 h-4 w-4" />
                    {u.first_name} {u.last_name}
                  </div>
                ))}
              </div>
            ) : <p className="text-sm text-gray-400 italic">No records found.</p>}
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 gap-8 lg:grid-cols-2">
        {/* Attendance Rates Chart Visual (Placeholder logic with CSS) */}
        <div className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mb-6 flex items-center justify-between">
            <h2 className="text-xl font-bold">Attendance Distribution</h2>
            <PieChart className="h-5 w-5 text-gray-400" />
          </div>
          <div className="space-y-4">
            {rateStats.map((item) => (
              <div key={item.label}>
                <div className="mb-1 flex justify-between text-sm">
                  <span className="text-gray-600 dark:text-gray-400">{item.label}</span>
                  <span className="font-bold">{item.value}</span>
                </div>
                <div className="h-2 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
                  <div 
                    className={`h-full ${item.color.replace('text', 'bg')}`} 
                    style={{ width: item.value }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Upcoming Birthdays */}
        <div className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mb-6 flex items-center justify-between">
            <h2 className="text-xl font-bold">Upcoming Birthdays</h2>
            <Gift className="h-5 w-5 text-pink-500" />
          </div>
          <div className="max-h-[300px] overflow-y-auto pr-2">
            {upcomingBirthdays.length > 0 ? (
              <div className="space-y-4">
                {upcomingBirthdays.map((user) => (
                  <div key={user.id} className="flex items-center justify-between rounded-xl bg-gray-50 p-4 dark:bg-gray-700/50">
                    <div className="flex items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-pink-100 text-pink-600 dark:bg-pink-900/30 dark:text-pink-400">
                        <Cake className="h-5 w-5" />
                      </div>
                      <div className="ml-3">
                        <p className="font-bold text-gray-800 dark:text-white">{user.first_name} {user.last_name}</p>
                        <p className="text-xs text-gray-500">{user.dob ? new Date(user.dob).toLocaleDateString() : 'N/A'}</p>
                      </div>
                    </div>
                    <span className="rounded-lg bg-pink-100 px-2 py-1 text-xs font-medium text-pink-700 dark:bg-pink-900/50 dark:text-pink-300">
                      Soon!
                    </span>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-center py-8 text-gray-500 italic">No birthdays coming up this month.</p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default AnalyticsPage;
