export type Role = "Admin" | "User" | "Technical";
export type Location = "DOA" | "CHIDA" | "OTHER";

export interface GeoPoint {
  lat: number;
  lng: number;
}

export interface UserDto {
  id: string;
  username?: string;
  first_name: string;
  last_name: string;
  email: string;
  dob?: string;
  avatar_url?: string;
  created_at: string;
  last_seen?: string;
  year_joined: string;
  reg_no: string;
  current_roster_hall?: string;
  current_roster_allocation?: string;
  role: Role;
  device_id?: string;
  gender?: string;
  phone?: string;
  address?: string;
  city?: string;
  state?: string;
  country?: string;
  is_active: boolean;
}

export interface Event {
  id: string;
  title: string;
  description: string;
  date: string;
  time: string;
  grace_period_in_minutes: number;
  attendance_type: string;
  location: Location;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface LoginPayload {
  user: string;
  password: string;
}

export interface CreateEventRequest {
  title: string;
  description: string;
  date: string;
  time: string;
  location: Location;
  attendance_type: string;
  grace_period_in_minutes: number;
}

export interface UpdateEventRequest {
  event_id: string;
  title?: string;
  description?: string;
  date?: string;
  time?: string;
  location?: Location;
  attendance_type?: string;
  grace_period_in_minutes?: number;
}

export interface CheckIntoEventRequest {
  event_id: string;
  user_id: string;
  attendance_type: string;
  location?: GeoPoint;
}

export interface CheckInWithIdentifierRequest {
  event_id: string;
  identifier: string;
  attendance_type: string;
  location?: GeoPoint;
}

export interface NewUser {
  first_name: string;
  last_name: string;
  email: string;
  password: string;
  dob?: string;
  year_joined: string;
  is_active: boolean;
  role: Role;
  gender?: string;
  phone?: string;
}

export interface UpdateUserRequest {
  id: string;
  first_name?: string;
  last_name?: string;
  dob?: string;
}

export interface UserFilter {
  page?: number;
  limit?: number;
  search?: string;
}

export interface ChangePasswordRequest {
  email: string;
  password: string;
}

export interface SignAttendanceRequest {
  location: GeoPoint;
  device_id: string;
}

export interface UserAttendanceDto {
  id: string;
  user_id: string;
  date: string;
  week_day: string;
  time_in: string;
  time_out?: string;
  marked_by?: string;
  event_id?: string;
  attendance_type: string;
  created_at: string;
  updated_at: string;
}

export interface UserPresentStats {
  absentees: UserDto[];
  date: string;
  presentees: UserDto[];
}

export interface AttendanceStats {
  admin_rate: number;
  user_rate: number;
  technical_rate: number;
  total_users: number;
  active_users: number;
  suspended_users: number;
}

export interface AttendanceSummary {
  total_days: number;
  days_present: number;
  rate: number;
}

export interface UserAttendanceHistory {
  user: UserDto;
  history: UserAttendanceDto[];
  summary: AttendanceSummary;
}

export interface Message<T = void> {
  message: string;
  data: T;
}

export interface EventAttendee {
  user_id: string;
  first_name: string;
  last_name: string;
  email: string;
  time_in: string; // NaiveTime as string
}

export interface EventStatsReport {
  total_attendees: number;
  eligible_attendees_count: number;
  attendees: EventAttendee[];
  absentees: UserDto[];
}
