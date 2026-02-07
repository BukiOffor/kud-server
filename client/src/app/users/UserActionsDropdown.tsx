'use client';

import { useState, useRef, useEffect } from 'react';
import { MoreVertical, UserCheck, History, UserX, UserPlus, Trash2, ShieldAlert, ShieldCheck } from 'lucide-react';
import { UserDto, Role } from '@/lib/types';

interface UserActionsDropdownProps {
  user: UserDto;
  onAttendance: (user: UserDto) => void;
  onViewHistory: (userId: string) => void;
  onDeactivate: (userId: string) => void;
  onActivate: (userId: string) => void;
  onUpdateRole: (userId: string, role: Role) => void;
  onDelete: (userId: string) => void;
}

const UserActionsDropdown = ({ 
  user, 
  onAttendance, 
  onViewHistory, 
  onDeactivate, 
  onActivate, 
  onUpdateRole,
  onDelete 
}: UserActionsDropdownProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  return (
    <div className="relative" ref={dropdownRef}>
      <button 
        onClick={() => setIsOpen(!isOpen)}
        className="rounded-full p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-900 dark:hover:bg-gray-700 dark:hover:text-white"
      >
        <MoreVertical className="h-5 w-5" />
      </button>

      {isOpen && (
        <div className="absolute right-0 z-20 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none dark:bg-gray-800 dark:ring-gray-700">
          <div className="py-1">
            <button
              onClick={() => {
                onAttendance(user);
                setIsOpen(false);
              }}
              className="flex w-full items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
            >
              <UserCheck className="mr-3 h-4 w-4 text-green-500" />
              Weekly Attendance
            </button>
            <button
              onClick={() => {
                onViewHistory(user.id);
                setIsOpen(false);
              }}
              className="flex w-full items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
            >
              <History className="mr-3 h-4 w-4 text-blue-500" />
              View History
            </button>
            
            <div className="my-1 border-t border-gray-100 dark:border-gray-700"></div>

            {/* Role Management */}
            {user.role === 'Admin' ? (
              <button
                onClick={() => {
                  if (confirm(`Are you sure you want to demote ${user.first_name} to a regular User?`)) {
                    onUpdateRole(user.id, 'User');
                  }
                  setIsOpen(false);
                }}
                className="flex w-full items-center px-4 py-2 text-sm text-amber-600 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <ShieldAlert className="mr-3 h-4 w-4" />
                Demote to User
              </button>
            ) : (
              <button
                onClick={() => {
                  if (confirm(`Are you sure you want to promote ${user.first_name} to an Admin?`)) {
                    onUpdateRole(user.id, 'Admin');
                  }
                  setIsOpen(false);
                }}
                className="flex w-full items-center px-4 py-2 text-sm text-indigo-600 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <ShieldCheck className="mr-3 h-4 w-4" />
                Promote to Admin
              </button>
            )}

            <div className="my-1 border-t border-gray-100 dark:border-gray-700"></div>

            {user.is_active ? (
              <button
                onClick={() => {
                  onDeactivate(user.id);
                  setIsOpen(false);
                }}
                className="flex w-full items-center px-4 py-2 text-sm text-orange-600 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <UserX className="mr-3 h-4 w-4" />
                Suspend User
              </button>
            ) : (
              <button
                onClick={() => {
                  onActivate(user.id);
                  setIsOpen(false);
                }}
                className="flex w-full items-center px-4 py-2 text-sm text-blue-600 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <UserPlus className="mr-3 h-4 w-4" />
                Recall User
              </button>
            )}

            <button
              onClick={() => {
                if (confirm(`Are you sure you want to delete ${user.first_name} ${user.last_name}?`)) {
                  onDelete(user.id);
                }
                setIsOpen(false);
              }}
              className="flex w-full items-center px-4 py-2 text-sm text-red-600 hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              <Trash2 className="mr-3 h-4 w-4" />
              Delete User
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default UserActionsDropdown;
