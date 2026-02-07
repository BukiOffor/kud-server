import axios from 'axios';
import { 
  LoginPayload, UserDto, Event, CheckIntoEventRequest, 
  NewUser, UserFilter, UpdateUserRequest, Message,
  CreateEventRequest, UpdateEventRequest, ChangePasswordRequest,
  SignAttendanceRequest, UserAttendanceDto, UserPresentStats,
  AttendanceStats, AttendanceSummary, UserAttendanceHistory
} from './types';

const API_BASE_URL ='https://api.koinoniaushers.cloud/api/v1';

//const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || ' https://5e188efecc5a.ngrok-free.app/api/v1';

const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add interceptor to include token in requests
// api.interceptors.request.use((config) => {
//   if (typeof window !== 'undefined') {
//     const token = localStorage.getItem('auth_token');
//     if (token) {
//       config.headers.Authorization = `Bearer ${token}`;
//     }
//   }
//   return config;
// });


export const authApi = {
  login: (payload: LoginPayload) => api.post<UserDto>('/auth/login', payload),
};

export const eventsApi = {
  getAll: () => api.get<Event[]>('/events'),
  getUpcoming: () => api.get<Event[]>('/events/upcoming'),
  getPast: () => api.get<Event[]>('/events/past'),
  getById: (id: string) => api.get<Event>(`/events/get/${id}`),
  getByUser: (userId: string) => api.get<Event[]>(`/events/user/${userId}`),
  create: (payload: CreateEventRequest) => api.post<Event>('/events/create', payload),
  update: (payload: UpdateEventRequest) => api.patch<Event>('/events/update', payload),
  delete: (id: string) => api.delete<Message>(`/events/delete/${id}`),
  checkIn: (payload: CheckIntoEventRequest) => api.post<Message>('/events/attendance/check-in', payload),
};

export const usersApi = {
  getById: (id: string) => api.post<UserDto>(`/users/get/${id}`),
  getAll: () => api.get<UserDto[]>('/users/admin/get_all'),
  register: (payload: NewUser) => api.post<Message>('/users/admin/register', payload),
  update: (payload: UpdateUserRequest) => api.post<Message>('/users/update', payload),
  delete: (id: string) => api.post<Message>(`/users/admin/delete/${id}`),
  deactivate: (id: string) => api.post<Message>(`/users/admin/deactive/${id}`),
  changePassword: (payload: ChangePasswordRequest) => api.post<Message>('/users/change_password', payload),
  importUsers: (formData: FormData) => api.post<Message>('/users/admin/import', formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
    },
  }),
};

export const attendanceApi = {
  sign: (payload: SignAttendanceRequest) => api.post<Message>('/attendance/check-in', payload),
  adminSign: (userId: string) => api.get<Message>(`/attendance/admin/sign/${userId}`),
};

export const analyticsApi = {
  getTotalUsers: () => api.get<Message<UserDto[]>>('/analytics/total-users'),
  getUsersPresentOnDay: (date: string) => api.get<Message<UserPresentStats>>(`/analytics/users-on-day`, { params: { date } }),
  getUpcomingBirthdays: () => api.get<Message<UserDto[]>>('/analytics/upcoming-birthdays'),
  getAttendanceRates: () => api.get<Message<AttendanceStats>>('/analytics/attendance-rates'),
  getUserAttendance: (userId: string) => api.get<Message<UserAttendanceHistory>>(`/analytics/user-attendance/${userId}`),
};

export default api;
