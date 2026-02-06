'use client';

import { useState, useEffect, useRef } from 'react';
import { usersApi } from '@/lib/api';
import { UserDto } from '@/lib/types';
import { User, Mail, Shield, Search, UserPlus, MoreVertical, Download, Upload, Loader2 } from 'lucide-react';
import Modal from '@/components/ui/Modal';
import RegisterUserForm from './RegisterUserForm';

const UsersPage = () => {
  const [users, setUsers] = useState<UserDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [importing, setImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [isRegisterModalOpen, setIsRegisterModalOpen] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    fetchUsers();
  }, []);

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

  const filteredUsers = users.filter(user => 
    `${user.first_name} ${user.last_name}`.toLowerCase().includes(search.toLowerCase()) ||
    user.email.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div>
      <input 
        type="file" 
        ref={fileInputRef} 
        onChange={handleFileChange} 
        accept=".csv" 
        className="hidden" 
      />
      <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Users Management</h1>
        <div className="flex flex-wrap gap-2">
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
                    <span className="inline-flex h-2.5 w-2.5 rounded-full bg-green-500 me-2"></span>
                    Active
                  </td>
                  <td className="px-6 py-4 text-right">
                    <button className="text-gray-400 hover:text-gray-900 dark:hover:text-white">
                      <MoreVertical className="h-5 w-5" />
                    </button>
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
          }} 
          onCancel={() => setIsRegisterModalOpen(false)} 
        />
      </Modal>
    </div>
  );
};

export default UsersPage;
