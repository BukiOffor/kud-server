'use client';

import { useState, useEffect } from 'react';
import { analyticsApi, eventsApi, attendanceApi } from '@/lib/api';
import { AttendanceStats, Event, UserDto, UserAttendanceHistory } from '@/lib/types';
import { 
  Users, Calendar, CheckSquare, Clock, Loader2, 
  ArrowUpRight, ChevronRight, Gift, Cake, Activity, Shield,
  UserCheck, UserX
} from 'lucide-react';
import Link from 'next/link';

const Dashboard = () => {
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState<AttendanceStats | null>(null);
  const [userHistory, setUserHistory] = useState<UserAttendanceHistory | null>(null);
  const [upcomingEvents, setUpcomingEvents] = useState<Event[]>([]);
  const [upcomingBirthdays, setUpcomingBirthdays] = useState<UserDto[]>([]);
  const [user, setUser] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      setUser(JSON.parse(storedUser));
    }
  }, []);

  useEffect(() => {
    if (user) {
      fetchDashboardData();
    }
  }, [user]);

  const isAdminOrTech = user?.role === 'Admin' || user?.role === 'Technical';

  const fetchDashboardData = async () => {
    try {
      setLoading(true);
      const promises: Promise<any>[] = [
        eventsApi.getUpcoming(),
        analyticsApi.getUpcomingBirthdays()
      ];

      if (isAdminOrTech) {
        promises.push(analyticsApi.getAttendanceRates());
      } else if (user?.id) {
        promises.push(analyticsApi.getUserAttendance(user.id));
      }

      const results = await Promise.all(promises);
      
      console.log(results);
      setUpcomingEvents(results[0].data.slice(0, 5));
      setUpcomingBirthdays(results[1].data.data);

      if (isAdminOrTech) {
        setStats(results[2]?.data?.data);
      } else {
        setUserHistory(results[2]?.data?.data);
      }

      setError(null);
    } catch (err) {
      console.error('Failed to fetch dashboard data:', err);
    } finally {
      setLoading(false);
    }
  };

  const statCards = isAdminOrTech ? [
    { 
      name: 'Total Users', 
      value: stats?.total_users?.toString() || '...', 
      icon: Users, 
      color: 'text-blue-600', 
      bg: 'bg-blue-100 dark:bg-blue-900/30 dark:text-blue-400' 
    },
    { 
      name: 'Admin Attendance Rate', 
      value: stats ? `${stats.admin_rate.toFixed(1)}%` : '...', 
      icon: Shield, 
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
      name: "User's Attendance Rate", 
      value: stats ? `${stats.user_rate.toFixed(1)}%` : '...', 
      icon: Clock, 
      color: 'text-orange-600', 
      bg: 'bg-orange-100 dark:bg-orange-900/30 dark:text-orange-400' 
    },
  ] : [
    { 
      name: 'My Attendance', 
      value: userHistory ? `${userHistory.summary.rate.toFixed(1)}%` : '...', 
      icon: Activity, 
      color: 'text-blue-600', 
      bg: 'bg-blue-100 dark:bg-blue-900/30 dark:text-blue-400' 
    },
    { 
      name: 'Days Present', 
      value: userHistory ? userHistory.summary.days_present.toString() : '...', 
      icon: CheckSquare, 
      color: 'text-green-600', 
      bg: 'bg-green-100 dark:bg-green-900/30 dark:text-green-400' 
    },
    { 
      name: 'Total Tracking', 
      value: userHistory ? userHistory.summary.total_days.toString() : '...', 
      icon: Calendar, 
      color: 'text-purple-600', 
      bg: 'bg-purple-100 dark:bg-purple-900/30 dark:text-purple-400' 
    },
    { 
      name: 'Birthdays', 
      value: upcomingBirthdays.length.toString(), 
      icon: Gift, 
      color: 'text-pink-600', 
      bg: 'bg-pink-100 dark:bg-pink-900/30 dark:text-pink-400' 
    },
  ];

  const isBirthdayToday = (dobString: string | undefined) => {
    if (!dobString) return false;
    const dob = new Date(dobString);
    const today = new Date();
    return dob.getDate() === today.getDate() && 
           dob.getMonth() === today.getMonth();
  };

  const isEventActive = (dateStr: string, timeStr: string, graceMinutes: number) => {
    try {
      const start = new Date(`${dateStr}T${timeStr}`);
      const end = new Date(start.getTime() + (graceMinutes || 30) * 60000);
      const now = new Date();
      return now >= start && now <= end;
    } catch {
      return false;
    }
  };

  const handleSelfCheckIn = async (event: Event) => {
    if (!user) {
      alert('Please log in to check in.');
      return;
    }

    setLoading(true);
    try {
      let location: any = undefined;
      
      try {
        const pos: any = await new Promise((resolve, reject) => {
          navigator.geolocation.getCurrentPosition(resolve, reject, { 
            timeout: 10000,
            enableHighAccuracy: true
          });
        });
        location = {
          lat: pos.coords.latitude,
          lng: pos.coords.longitude
        };
      } catch (err) {
        console.warn('Geolocation failed:', err);
      }

      await eventsApi.checkIn({
        event_id: event.id,
        user_id: user.id || user.user_id,
        attendance_type: 'Standard',
        location
      });
      
      alert('Successfully checked in!');
      fetchDashboardData();
    } catch (err: any) {
      alert(err.response?.data?.message || 'Failed to check in. Please ensure you are at the correct location.');
    } finally {
      setLoading(false);
    }
  };

  const todaysBirthdays = upcomingBirthdays.filter(u => isBirthdayToday(u.dob));

  return (
    <div>
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-800 dark:text-white">Dashboard Overview</h1>
          <p className="mt-1 text-gray-500 dark:text-gray-400">Welcome back, {user?.first_name}! Here&apos;s what&apos;s happening today.</p>
        </div>
        <Link 
          href="/attendance"
          className="flex items-center rounded-xl bg-blue-600 px-6 py-3 text-sm font-bold text-white shadow-lg shadow-blue-500/30 transition-all hover:bg-blue-700"
        >
          Sign Attendance
          <ArrowUpRight className="ml-2 h-4 w-4" />
        </Link>
      </div>

      {/* Birthday Banner */}
      {todaysBirthdays.length > 0 && (
        <div className="mb-8 overflow-hidden rounded-2xl border border-pink-100 bg-gradient-to-r from-pink-500 to-rose-500 p-1 shadow-lg shadow-pink-500/20">
          <div className="flex flex-col items-center justify-between gap-4 rounded-xl bg-white/10 px-6 py-4 backdrop-blur-sm sm:flex-row">
            <div className="flex items-center">
              <div className="mr-4 flex h-12 w-12 items-center justify-center rounded-full bg-white/20 text-white animate-bounce">
                <Cake className="h-6 w-6" />
              </div>
              <div>
                <h2 className="text-lg font-bold text-white">Today&apos;s Celebration!</h2>
                <p className="text-sm text-pink-50">
                  {todaysBirthdays.map(u => `${u.first_name} ${u.last_name}`).join(', ')} {todaysBirthdays.length > 1 ? 'are' : 'is'} celebrating their birthday today! 🎂
                </p>
              </div>
            </div>
            <div className="rounded-lg bg-white/20 px-4 py-2 text-sm font-bold text-white">
              Wish them well! 🎉
            </div>
          </div>
        </div>
      )}
      
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

      <div className="mt-10 grid grid-cols-1 gap-8 lg:grid-cols-3">
        {/* Recent Activities / My History */}
        <div className="lg:col-span-2 rounded-2xl border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mb-6 flex items-center justify-between">
            <h2 className="text-xl font-bold text-gray-800 dark:text-white">
              {isAdminOrTech ? 'Community Birthdays' : 'My Recent Check-ins'}
            </h2>
            {isAdminOrTech ? (
               <div className="flex items-center rounded-lg bg-pink-50 px-3 py-1 text-xs font-medium text-pink-700 dark:bg-pink-900/30 dark:text-pink-400">
                 <Gift className="mr-1 h-3 w-3" />
                 Celebrations
               </div>
            ) : (
              <Link href="/analytics" className="text-sm font-medium text-blue-600 hover:underline">View History</Link>
            )}
          </div>
          
          <div className="space-y-4">
            {loading ? (
              <div className="flex justify-center py-10">
                <Loader2 className="h-8 w-8 animate-spin text-gray-300" />
              </div>
            ) : isAdminOrTech ? (
              // Admin View - Birthdays prioritize
              upcomingBirthdays.length > 0 ? (
                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  {upcomingBirthdays.map(bUser => {
                    const isToday = isBirthdayToday(bUser.dob);
                    return (
                      <div key={bUser.id} className={`flex items-center rounded-xl p-4 ${isToday ? 'bg-pink-50 border border-pink-100 dark:bg-pink-900/10 dark:border-pink-900/30' : 'bg-gray-50 dark:bg-gray-700/50'}`}>
                        <div className={`flex h-10 w-10 items-center justify-center rounded-full ${isToday ? 'bg-pink-500 text-white animate-pulse' : 'bg-pink-100 text-pink-600 dark:bg-pink-900/30 dark:text-pink-400'}`}>
                          <Cake className="h-5 w-5" />
                        </div>
                        <div className="ml-3">
                          <div className="flex items-center gap-2">
                            <p className="font-bold text-sm text-gray-800 dark:text-white">{bUser.first_name} {bUser.last_name}</p>
                            {isToday && (
                              <span className="rounded-full bg-pink-100 px-2 py-0.5 text-[10px] font-bold text-pink-600 dark:bg-pink-900/30 dark:text-pink-400">
                                TODAY
                              </span>
                            )}
                          </div>
                          <p className="text-xs text-gray-500 dark:text-gray-400">Birthday: {bUser.dob ? new Date(bUser.dob).toLocaleDateString() : 'N/A'}</p>
                        </div>
                      </div>
                    );
                  })}
                </div>
              ) : (
                <p className="text-center py-10 text-gray-500 italic dark:text-gray-400">No birthdays this month.</p>
              )
            ) : (
              // User View - Recent history
              userHistory?.history.length ? (
                <div className="space-y-3">
                  {userHistory.history.slice(0, 5).map(item => (
                    <div key={item.id} className="flex items-center justify-between rounded-xl bg-gray-50 p-4 dark:bg-gray-700/50">
                      <div className="flex items-center">
                        <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400">
                          <CheckSquare className="h-5 w-5" />
                        </div>
                        <div className="ml-3">
                          <p className="font-bold text-sm text-gray-800 dark:text-white">{item.date}</p>
                          <p className="text-xs text-gray-500 dark:text-gray-400">{item.time_in} • {item.attendance_type}</p>
                        </div>
                      </div>
                      <span className="text-xs font-semibold text-gray-400 uppercase">{item.week_day}</span>
                    </div>
                  ))}
                </div>
              ) : (
                <p className="text-center py-10 text-gray-500 italic dark:text-gray-400">No check-in history found.</p>
              )
            )}
          </div>
        </div>

        {/* Universal Section: Upcoming Events */}
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
              upcomingEvents.map(event => {
                const isActive = isEventActive(event.date, event.time, event.grace_period_in_minutes);
                return (
                  <div key={event.id} className="flex items-center justify-between rounded-xl bg-gray-50 p-4 dark:bg-gray-700/50">
                    <div className="flex items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400">
                        <Calendar className="h-5 w-5" />
                      </div>
                      <div className="ml-3">
                        <p className="font-bold text-sm text-gray-800 dark:text-white">{event.title}</p>
                        <p className="text-xs text-gray-500 dark:text-gray-400">{event.date} • {event.time}</p>
                      </div>
                    </div>
                    {isActive ? (
                      <button 
                        onClick={() => handleSelfCheckIn(event)}
                        className="flex items-center rounded-lg bg-green-600 px-3 py-1.5 text-xs font-bold text-white hover:bg-green-700 shadow-sm shadow-green-500/20"
                      >
                        <UserCheck className="mr-1.5 h-3.5 w-3.5" />
                        Check In
                      </button>
                    ) : (
                      <ChevronRight className="h-4 w-4 text-gray-300" />
                    )}
                  </div>
                );
              })
            ) : (
              <p className="text-center py-10 text-gray-500 italic dark:text-gray-400">No upcoming events.</p>
            )}

            {!isAdminOrTech && upcomingBirthdays.length > 0 && (
              <div className="mt-8 border-t border-gray-100 pt-6 dark:border-gray-700">
                <h3 className="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-400">Birthdays This Month</h3>
                <div className="space-y-3">
                  {upcomingBirthdays.map(bUser => {
                    const isToday = isBirthdayToday(bUser.dob);
                    return (
                      <div key={bUser.id} className="flex items-center justify-between">
                         <div className="flex items-center">
                           <div className={`mr-3 flex h-8 w-8 items-center justify-center rounded-full ${isToday ? 'bg-pink-500 text-white' : 'bg-pink-50 text-pink-500'}`}>
                             <Cake className="h-4 w-4" />
                           </div>
                           <p className={`text-sm font-medium ${isToday ? 'text-pink-600 dark:text-pink-400 font-bold' : ''}`}>
                             {bUser.first_name} {bUser.last_name}
                           </p>
                         </div>
                         {isToday && (
                           <span className="rounded-full bg-pink-100 px-2 py-0.5 text-[10px] font-bold text-pink-600 dark:bg-pink-900/30 dark:text-pink-400">
                             TODAY
                           </span>
                         )}
                      </div>
                    );
                  })}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
