import axios from 'axios';
import { 
  LoginPayload, UserDto, Event, CheckIntoEventRequest, 
  NewUser, UserFilter, UpdateUserRequest, Message,
  CreateEventRequest, UpdateEventRequest, ChangePasswordRequest
} from './types';

const API_BASE_URL ='http://localhost:9898/api/v1';
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
  getAll: (filter: UserFilter) => api.post<UserDto[]>('/users/get_all', filter),
  register: (payload: NewUser) => api.post<Message>('/users/register', payload),
  update: (payload: UpdateUserRequest) => api.post<Message>('/users/update', payload),
  delete: (id: string) => api.post<Message>(`/users/delete/${id}`),
  deactivate: (id: string) => api.post<Message>(`/users/deactive/${id}`),
  changePassword: (payload: ChangePasswordRequest) => api.post<Message>('/users/change_password', payload),
};

export default api;
