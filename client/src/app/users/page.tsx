'use client';

import { useState, useEffect, useRef } from 'react';
import { usersApi, analyticsApi } from '@/lib/api';
import { UserDto, AttendanceStats, Role } from '@/lib/types';
import { User, Mail, Shield, Search, UserPlus, MoreVertical, Download, Upload, Loader2, Users, UserCheck, UserX } from 'lucide-react';
import Modal from '@/components/ui/Modal';
import RegisterUserForm from './RegisterUserForm';
import UserDetailsModal from './UserDetailsModal';
import MarkUserAttendanceModal from './MarkUserAttendanceModal';
import UserActionsDropdown from './UserActionsDropdown';

const UsersPage = () => {
  const [users, setUsers] = useState<UserDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [importing, setImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [isRegisterModalOpen, setIsRegisterModalOpen] = useState(false);
  const [isDetailsModalOpen, setIsDetailsModalOpen] = useState(false);
  const [isAttendanceModalOpen, setIsAttendanceModalOpen] = useState(false);
  const [selectedUserId, setSelectedUserId] = useState<string | null>(null);
  const [selectedUserName, setSelectedUserName] = useState<string>('');
  const [statusFilter, setStatusFilter] = useState<'All' | 'Active' | 'Inactive'>('All');
  const [stats, setStats] = useState<AttendanceStats | null>(null);
  const [statsLoading, setStatsLoading] = useState(true);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    fetchUsers();
    fetchGlobalStats();
  }, []);

  const fetchGlobalStats = async () => {
    try {
      setStatsLoading(true);
      const response = await analyticsApi.getAttendanceRates();
      setStats(response.data.data);
    } catch (err) {
      console.error('Failed to fetch global stats:', err);
    } finally {
      setStatsLoading(false);
    }
  };

  const handleOpenDetails = (userId: string) => {
    setSelectedUserId(userId);
    setIsDetailsModalOpen(true);
  };

  const handleOpenAttendance = (user: UserDto) => {
    setSelectedUserId(user.id);
    setSelectedUserName(`${user.first_name} ${user.last_name}`);
    setIsAttendanceModalOpen(true);
  };

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const response = await usersApi.getAll();
      setUsers(response.data);
      setError(null);
    } catch (err) {
      console.error('Failed to fetch users:', err);
      setError('Failed to load users. Please check backend connection.');
    } finally {
      setLoading(false);
    }
  };

  const handleDeactivateUser = async (userId: string) => {
    try {
      await usersApi.deactivate(userId);
      alert('User deactivated (suspended) successfully');
      fetchUsers();
      fetchGlobalStats();
    } catch (err) {
      alert('Failed to deactivate user');
    }
  };

  const handleActivateUser = async (userId: string) => {
    try {
      await usersApi.activate(userId);
      alert('User activated (recalled) successfully');
      fetchUsers();
      fetchGlobalStats();
    } catch (err) {
      alert('Failed to activate user');
    }
  };

  const handleUpdateRole = async (userId: string, role: Role) => {
    try {
      await usersApi.updateRole(userId, role);
      alert(`User role updated to ${role} successfully`);
      fetchUsers();
      fetchGlobalStats();
    } catch (err) {
      alert('Failed to update user role');
    }
  };

  const handleDeleteUser = async (userId: string) => {
    try {
      await usersApi.delete(userId);
      alert('User deleted successfully');
      fetchUsers();
      fetchGlobalStats();
    } catch (err) {
      alert('Failed to delete user');
    }
  };

  const handleExport = async () => {
    try {
       // Logic for handling binary file download
       alert('Exporting users... (Handled by backend endpoint /users/export)');
    } catch (err) {
       alert('Failed to export users');
    }
  };

  const handleImportClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    if (file.type !== 'text/csv' && !file.name.endsWith('.csv')) {
      alert('Please upload a valid CSV file.');
      return;
    }

    const formData = new FormData();
    formData.append('file', file);

    try {
      setImporting(true);
      const response = await usersApi.importUsers(formData);
      alert(response.data.message || 'Users imported successfully');
      fetchUsers();
      fetchGlobalStats();
    } catch (err) {
      console.error('Failed to import users:', err);
      alert('Failed to import users. Please check the file format.');
    } finally {
      setImporting(false);
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    }
  };

  const filteredUsers = users.filter(user => {
    const matchesSearch = `${user.first_name} ${user.last_name}`.toLowerCase().includes(search.toLowerCase()) ||
    user.email.toLowerCase().includes(search.toLowerCase());
    
    const matchesStatus = statusFilter === 'All' || 
      (statusFilter === 'Active' && user.is_active) || 
      (statusFilter === 'Inactive' && !user.is_active);
      
    return matchesSearch && matchesStatus;
  });

  return (
    <div>
      <input 
        type="file" 
        ref={fileInputRef} 
        onChange={handleFileChange} 
        accept=".csv" 
        className="hidden" 
      />
      
      {/* Header & Actions */}
      <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Users Management</h1>
        <div className="flex flex-wrap gap-2">
          {/* Search */}
          <div className="relative min-w-[200px]">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
            <input
              type="text"
              placeholder="Search users..."
              className="w-full rounded-lg border border-gray-200 bg-white py-2 pl-10 pr-4 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-700 dark:bg-gray-800 dark:text-white"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>

          {/* Status Filter */}
          <select 
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value as any)}
            className="rounded-lg border border-gray-200 bg-white px-4 py-2 text-sm text-gray-700 focus:border-blue-500 focus:outline-none dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300"
          >
            <option value="All">All Statuses</option>
            <option value="Active">Active Only</option>
            <option value="Inactive">Inactive Only</option>
          </select>
          <button 
            onClick={handleExport}
            className="flex items-center rounded-lg border border-gray-200 bg-white px-4 py-2 text-gray-700 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
          >
            <Download className="me-2 h-4 w-4" />
            Export
          </button>
          <button 
            onClick={handleImportClick}
            disabled={importing}
            className="flex items-center rounded-lg border border-gray-200 bg-white px-4 py-2 text-gray-700 hover:bg-gray-50 disabled:opacity-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
          >
            {importing ? (
              <Loader2 className="me-2 h-4 w-4 animate-spin" />
            ) : (
              <Upload className="me-2 h-4 w-4" />
            )}
            Import
          </button>
          <button 
            onClick={() => setIsRegisterModalOpen(true)}
            className="flex items-center rounded-lg bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
          >
            <UserPlus className="me-2 h-5 w-5" />
            Add User
          </button>
        </div>
      </div>

      {/* Stats Overview */}
      <div className="mb-8 grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div className="flex items-center rounded-2xl border border-gray-200 bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mr-4 rounded-xl bg-blue-100 p-3 dark:bg-blue-900/30">
            <Users className="h-6 w-6 text-blue-600 dark:text-blue-400" />
          </div>
          <div>
            <p className="text-xs font-medium text-gray-500 dark:text-gray-400">Total Users</p>
            <h3 className="text-xl font-bold text-gray-800 dark:text-white">
              {statsLoading ? <Loader2 className="h-4 w-4 animate-spin opacity-20" /> : stats?.total_users || 0}
            </h3>
          </div>
        </div>
        <div className="flex items-center rounded-2xl border border-gray-200 bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mr-4 rounded-xl bg-green-100 p-3 dark:bg-green-900/30">
            <UserCheck className="h-6 w-6 text-green-600 dark:text-green-400" />
          </div>
          <div>
            <p className="text-xs font-medium text-gray-500 dark:text-gray-400">Active Users</p>
            <h3 className="text-xl font-bold text-gray-800 dark:text-white">
              {statsLoading ? <Loader2 className="h-4 w-4 animate-spin opacity-20" /> : stats?.active_users || 0}
            </h3>
          </div>
        </div>
        <div className="flex items-center rounded-2xl border border-gray-200 bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <div className="mr-4 rounded-xl bg-red-100 p-3 dark:bg-red-900/30">
            <UserX className="h-6 w-6 text-red-600 dark:text-red-400" />
          </div>
          <div>
            <p className="text-xs font-medium text-gray-500 dark:text-gray-400">Inactive Users</p>
            <h3 className="text-xl font-bold text-gray-800 dark:text-white">
              {statsLoading ? <Loader2 className="h-4 w-4 animate-spin opacity-20" /> : stats?.suspended_users || 0}
            </h3>
          </div>
        </div>
      </div>

      {error && (
        <div className="mb-4 rounded-lg bg-red-100 p-4 text-red-700 dark:bg-red-900/30 dark:text-red-400">
          {error}
        </div>
      )}

      <div className="overflow-x-auto rounded-lg border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <table className="w-full text-left text-sm text-gray-500 dark:text-gray-400">
          <thead className="bg-gray-50 text-xs uppercase text-gray-700 dark:bg-gray-700 dark:text-gray-400">
            <tr>
              <th className="px-6 py-3">User</th>
              <th className="px-6 py-3">Role</th>
              <th className="px-6 py-3">Reg No</th>
              <th className="px-6 py-3">Year Joined</th>
              <th className="px-6 py-3">Status</th>
              <th className="px-6 py-3">
                <span className="sr-only">Actions</span>
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
            {loading ? (
              <tr>
                <td colSpan={6} className="px-6 py-4 text-center">Loading users...</td>
              </tr>
            ) : filteredUsers.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-6 py-4 text-center">No users found</td>
              </tr>
            ) : (
              filteredUsers.map((user) => (
                <tr key={user.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                  <td className="flex items-center px-6 py-4">
                    <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100 text-blue-600">
                      <User className="h-6 w-6" />
                    </div>
                    <div className="ms-3">
                      <div className="text-base font-semibold text-gray-900 dark:text-white">
                        {user.first_name} {user.last_name}
                      </div>
                      <div className="flex items-center text-xs text-gray-500">
                        <Mail className="me-1 h-3 w-3" />
                        {user.email}
                      </div>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <span className="flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-300">
                      <Shield className="me-1 h-3 w-3" />
                      {user.role}
                    </span>
                  </td>
                  <td className="px-6 py-4 font-mono">{user.reg_no}</td>
                  <td className="px-6 py-4">{user.year_joined}</td>
                  <td className="px-6 py-4">
                      {user.is_active ? (
                        <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800 dark:bg-green-900/20 dark:text-green-400">
                          <span className="me-1 h-1.5 w-1.5 rounded-full bg-green-500"></span>
                          Active
                        </span>
                      ) : (
                        <span className="inline-flex items-center rounded-full bg-red-100 px-2.5 py-0.5 text-xs font-medium text-red-800 dark:bg-red-900/20 dark:text-red-400">
                          <span className="me-1 h-1.5 w-1.5 rounded-full bg-red-500"></span>
                          Inactive
                        </span>
                      )}
                    </td>
                  <td className="px-6 py-4 text-right">
                    <div className="flex items-center justify-end">
                        <UserActionsDropdown 
                          user={user}
                          onAttendance={handleOpenAttendance}
                          onViewHistory={handleOpenDetails}
                          onDeactivate={handleDeactivateUser}
                          onActivate={handleActivateUser}
                          onUpdateRole={handleUpdateRole}
                          onDelete={handleDeleteUser}
                        />
                      </div>
                    </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      <Modal 
        isOpen={isRegisterModalOpen} 
        onClose={() => setIsRegisterModalOpen(false)} 
        title="Register New User"
      >
        <RegisterUserForm 
          onSuccess={() => {
            setIsRegisterModalOpen(false);
            fetchUsers();
            fetchGlobalStats();
          }} 
          onCancel={() => setIsRegisterModalOpen(false)} 
        />
      </Modal>

      {selectedUserId && (
        <UserDetailsModal 
          isOpen={isDetailsModalOpen}
          onClose={() => {
            setIsDetailsModalOpen(false);
            setSelectedUserId(null);
          }}
          userId={selectedUserId}
        />
      )}

      {selectedUserId && isAttendanceModalOpen && (
        <MarkUserAttendanceModal
          isOpen={isAttendanceModalOpen}
          onClose={() => {
            setIsAttendanceModalOpen(false);
            setSelectedUserId(null);
          }}
          userId={selectedUserId}
          userName={selectedUserName}
        />
      )}
    </div>
  );
};

export default UsersPage;
