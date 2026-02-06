'use client';

import { Users, Calendar, CheckSquare, Clock } from 'lucide-react';

const Dashboard = () => {
  const stats = [
    { name: 'Total Users', value: '250', icon: Users, color: 'text-blue-600', bg: 'bg-blue-100' },
    { name: 'Active Events', value: '12', icon: Calendar, color: 'text-green-600', bg: 'bg-green-100' },
    { name: 'Recent Check-ins', value: '45', icon: CheckSquare, color: 'text-purple-600', bg: 'bg-purple-100' },
    { name: 'Upcoming Events', value: '3', icon: Clock, color: 'text-orange-600', bg: 'bg-orange-100' },
  ];

  return (
    <div>
      <h1 className="mb-6 text-2xl font-bold text-gray-800 dark:text-white">Dashboard Overview</h1>
      
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <div key={stat.name} className="flex items-center rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
            <div className={`mr-4 rounded-full p-3 ${stat.bg}`}>
              <stat.icon className={`h-6 w-6 ${stat.color}`} />
            </div>
            <div>
              <p className="text-sm font-medium text-gray-500 dark:text-gray-400">{stat.name}</p>
              <h3 className="text-2xl font-bold text-gray-800 dark:text-white">{stat.value}</h3>
            </div>
          </div>
        ))}
      </div>

      <div className="mt-8 grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <h2 className="mb-4 text-xl font-bold text-gray-800 dark:text-white">Recent Activities</h2>
          <div className="space-y-4">
            <p className="text-gray-500 dark:text-gray-400">Loading activities...</p>
          </div>
        </div>
        <div className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <h2 className="mb-4 text-xl font-bold text-gray-800 dark:text-white">Upcoming Events</h2>
          <div className="space-y-4">
            <p className="text-gray-500 dark:text-gray-400">Loading events...</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
