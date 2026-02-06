'use client';

import { useState, useEffect } from 'react';
import { analyticsApi, eventsApi } from '@/lib/api';
import { AttendanceStats, Event } from '@/lib/types';
import { Users, Calendar, CheckSquare, Clock, Loader2, ArrowUpRight, ChevronRight } from 'lucide-react';
import Link from 'next/link';

const Dashboard = () => {
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState<AttendanceStats | null>(null);
  const [upcomingEvents, setUpcomingEvents] = useState<Event[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchDashboardData();
  }, []);

  const fetchDashboardData = async () => {
    try {
      setLoading(true);
      const [statsRes, eventsRes] = await Promise.all([
        analyticsApi.getAttendanceRates(),
        eventsApi.getUpcoming()
      ]);
      console.log(statsRes.data);
      //@ts-ignore
      setStats(statsRes.data.data);
      setUpcomingEvents(eventsRes.data.slice(0, 5)); // Show top 5
      setError(null);
    } catch (err) {
      console.error('Failed to fetch dashboard data:', err);
      // setError('Failed to load dashboard data.');
    } finally {
      setLoading(false);
    }
  };

  const statCards = [
    { 
      name: 'Total Users', 
      value: stats?.total_users?.toString() || '...', 
      icon: Users, 
      color: 'text-blue-600', 
      bg: 'bg-blue-100 dark:bg-blue-900/30 dark:text-blue-400' 
    },
    { 
      name: 'Avg Attendance', 
      value: stats ? `${((stats.admin_rate + stats.user_rate + stats.technical_rate) / 3).toFixed(1)}%` : '...', 
      icon: CheckSquare, 
      color: 'text-green-600', 
      bg: 'bg-green-100 dark:bg-green-900/30 dark:text-green-400' 
    },
    { 
      name: 'Upcoming Events', 
      value: upcomingEvents.length.toString(), 
      icon: Calendar, 
      color: 'text-purple-600', 
      bg: 'bg-purple-100 dark:bg-purple-900/30 dark:text-purple-400' 
    },
    { 
      name: 'User Rate', 
      value: stats ? `${stats.user_rate}%` : '...', 
      icon: Clock, 
      color: 'text-orange-600', 
      bg: 'bg-orange-100 dark:bg-orange-900/30 dark:text-orange-400' 
    },
  ];

  return (
    <div>
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-800 dark:text-white">Dashboard Overview</h1>
          <p className="mt-1 text-gray-500 dark:text-gray-400">Welcome back! Here&apos;s what&apos;s happening today.</p>
        </div>
        <Link 
          href="/attendance"
          className="flex items-center rounded-xl bg-blue-600 px-6 py-3 text-sm font-bold text-white shadow-lg shadow-blue-500/30 transition-all hover:bg-blue-700"
        >
          Sign Attendance
          <ArrowUpRight className="ml-2 h-4 w-4" />
        </Link>
      </div>
      
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        {statCards.map((stat) => (
          <div key={stat.name} className="flex items-center rounded-2xl border border-gray-200 bg-white p-6 shadow-sm transition-all hover:shadow-md dark:border-gray-700 dark:bg-gray-800">
            <div className={`mr-4 rounded-xl p-3 ${stat.bg}`}>
              <stat.icon className={`h-6 w-6 ${stat.color}`} />
            </div>
            <div>
              <p className="text-sm font-medium text-gray-500 dark:text-gray-400">{stat.name}</p>
              <h3 className="text-2xl font-bold text-gray-800 dark:text-white">
                {loading ? <Loader2 className="h-5 w-5 animate-spin opacity-20" /> : stat.value}
              </h3>
            </div>
          </div>
        ))}
      </div>

      <div className="mt-10 grid grid-cols-1 gap-8 lg:grid-cols-2">
        <div className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mb-6 flex items-center justify-between">
            <h2 className="text-xl font-bold text-gray-800 dark:text-white">Recent Activities</h2>
            <Link href="/analytics" className="text-sm font-medium text-blue-600 hover:underline">View All</Link>
          </div>
          <div className="space-y-6">
            {loading ? (
              <div className="flex justify-center py-10">
                <Loader2 className="h-8 w-8 animate-spin text-gray-300" />
              </div>
            ) : (
              <p className="text-center py-10 text-gray-500 italic dark:text-gray-400">No recent activity logs available.</p>
            )}
          </div>
        </div>

        <div className="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mb-6 flex items-center justify-between">
            <h2 className="text-xl font-bold text-gray-800 dark:text-white">Upcoming Events</h2>
            <Link href="/events" className="text-sm font-medium text-blue-600 hover:underline">View All</Link>
          </div>
          <div className="space-y-4">
            {loading ? (
              <div className="flex justify-center py-10">
                <Loader2 className="h-8 w-8 animate-spin text-gray-300" />
              </div>
            ) : upcomingEvents.length > 0 ? (
              upcomingEvents.map(event => (
                <div key={event.id} className="flex items-center justify-between rounded-xl bg-gray-50 p-4 dark:bg-gray-700/50">
                  <div className="flex items-center">
                    <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400">
                      <Calendar className="h-5 w-5" />
                    </div>
                    <div className="ml-3">
                      <p className="font-bold text-gray-800 dark:text-white">{event.title}</p>
                      <p className="text-xs text-gray-500 dark:text-gray-400">{event.date} • {event.time}</p>
                    </div>
                  </div>
                  <ChevronRight className="h-5 w-5 text-gray-300" />
                </div>
              ))
            ) : (
              <p className="text-center py-10 text-gray-500 italic dark:text-gray-400">No upcoming events found.</p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
