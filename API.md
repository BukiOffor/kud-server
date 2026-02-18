# API Documentation

## Overview

Base URL: `http://localhost:8000/api/v1` (or your deployed URL)
All endpoints return standard HTTP status codes.
Timestamps are generally returned as ISO 8601 strings (e.g., `2023-10-27T10:00:00`).

## Authentication (`/auth`)

### Login
Authenticate a user and retrieve their details.

- **Method:** `POST`
- **Path:** `/auth/login`
- **Request Body:** `LoginPayload`
- **Response:** `UserDto`
- **Note:** Sets a session cookie/token in the response headers/cookies.

---

## Events (`/events`)

### Check In to Event
Record user attendance for a specific event.

- **Method:** `POST`
- **Path:** `/events/attendance/check-in`
- **Request Body:** `CheckIntoEventRequest`
- **Response:** `Message`

### Check In with Identifier
Check into an event using email or Reg No (Admin only).

- **Method:** `POST`
- **Path:** `/events/attendance/check-in-identifier`
- **Request Body:** `CheckInWithIdentifierRequest`
- **Response:** `Message`

### Get Upcoming Events
Retrieve a list of events scheduled for the future.

- **Method:** `GET`
- **Path:** `/events/upcoming`
- **Response:** `Array<Event>`

### Get Past Events
Retrieve a list of past events.

- **Method:** `GET`
- **Path:** `/events/past`
- **Response:** `Array<Event>`

### Get All Events
Retrieve all events.

- **Method:** `GET`
- **Path:** `/events/`
- **Response:** `Array<Event>`

### Get Event by ID
Retrieve a single event.

- **Method:** `GET`
- **Path:** `/events/get/{event_id}`
- **Parameters:**
    - `event_id`: UUID
- **Response:** `Event`

### Get Events by User
Retrieve events associated with a specific user.

- **Method:** `GET`
- **Path:** `/events/user/{user_id}`
- **Parameters:**
    - `user_id`: UUID
- **Response:** `Array<Event>`

### Create Event
Create a new event (Admin only).

- **Method:** `POST`
- **Path:** `/events/create`
- **Request Body:** `CreateEventRequest`
- **Response:** `Event`

### Update Event
Update an existing event (Admin only).

- **Method:** `PATCH`
- **Path:** `/events/update`
- **Request Body:** `UpdateEventRequest`
- **Response:** `Event`

### Delete Event
Delete an event (Admin only).

- **Method:** `DELETE`
- **Path:** `/events/delete/{event_id}`
- **Parameters:**
    - `event_id`: UUID
- **Response:** `Message`

---

## Users (`/users`)

### Register User
Register a new user (Admin only).

- **Method:** `POST`
- **Path:** `/users/admin/register`
- **Request Body:** `NewUser`
- **Response:** `Message`

### Get User
Get user details.

- **Method:** `GET`
- **Path:** `/users/get/{id}`
- **Response:** `UserDto`

### Get All Users
Filter and retrieve users.

- **Method:** `GET`
- **Path:** `/users/admin/get_all`
- **Request Body:** `UserFilter`
- **Response:** `Array<UserDto>`

### Update User
Update user details.

- **Method:** `POST`
- **Path:** `/users/update`
- **Request Body:** `UpdateUserRequest`
- **Response:** `Message`

### Delete User
Delete a user (Admin only).

- **Method:** `DELETE`
- **Path:/` `/users/admin/delete/{id}`
- **Response:** `Message`

### Deactivate User
Deactivate a user (Admin only).

- **Method:** `PATCH`
- **Path:** `/users/admin/deactivate/{id}`
- **Response:** `Message`

### Activate User
Activate a user (Admin only).

- **Method:** `PATCH`
- **Path:** `/users/admin/activate/{id}`
- **Response:** `Message`

### Import Users
Import users via file (Admin only).

- **Method:** `POST`
- **Path:** `/users/admin/import`
- **Request Body:** `Multipart/Form-Data`
- **Response:** `Message`

### Export Users
Export users as a file (Admin only).

- **Method:** `GET`
- **Path:** `/users/admin/export`
- **Response:** Binary File

### Change Password
Change user password.

- **Method:** `POST`
- **Path:** `/users/change-password`
- **Request Body:** `ChangePasswordRequest`
- **Response:** `Message`

### Reset Device ID
Reset a user's device ID (Admin only).

- **Method:** `PATCH`
- **Path:** `/users/admin/reset-device-id/{id}`
- **Parameters:**
    - `id`: UUID
- **Response:** `Message`

---

## Attendance (`/attendance`)

### Sign Attendance
General user check-in (e.g. daily attendance).

- **Method:** `POST`
- **Path:** `/attendance/check-in`
- **Request Body:** `SignAttendanceRequest`
- **Response:** `Message`

### Admin Sign Attendance
Admin signs attendance for a worker.

- **Method:** `POST`
- **Path:** `/attendance/admin/sign/{id}`
- **Parameters:**
    - `id`: UUID (Worker ID)
- **Response:** `Message`

---

## Logs (`/logs`) (Admin Only)

### Get All Logs
Retrieve a paginated list of activity logs.

- **Method:** `GET`
- **Path:** `/logs`
- **Query Parameters:** `Pagination`
- **Response:** `PaginatedResult<ActivityLogResponse>`

### Get User Activity
Retrieve activity logs for a specific user.

- **Method:** `GET`
- **Path:** `/logs/user/{id}`
- **Parameters:**
    - `id`: UUID
- **Response:** `Array<ActivityLog>`

---

## Roster (`/roster`) (Admin Only)

### Create Roster
Create a new roster.

- **Method:** `POST`
- **Path:** `/roster/create`
- **Request Body:** `NewRoster`
- **Response:** `Message<Roster>`

### Get Roster
Retrieve a single roster.

- **Method:** `GET`
- **Path:** `/roster/{id}`
- **Parameters:**
    - `id`: UUID
- **Response:** `Roster`

### Update Roster
Update an existing roster.

- **Method:** `PATCH`
- **Path:** `/roster/update`
- **Request Body:** `UpdateRosterRequest`
- **Response:** `Message<Roster>`

### Get All Rosters
Retrieve all rosters.

- **Method:** `GET`
- **Path:** `/roster/all`
- **Response:** `Array<Roster>`

### Update User Hall
Update a user's hall assignment in a roster.

- **Method:** `PATCH`
- **Path:** `/roster/user-hall`
- **Request Body:** `UpdateUserHallRequest`
- **Response:** `Message<()>`

---

## Analytics (`/analytics`) (Admin Only)

### Get Total Users
Retrieve a list of all users as a status message.

- **Method:** `GET`
- **Path:** `/analytics/total-users`
- **Response:** `Message<Array<UserDto>>`

### Get Users Present on Day
Retrieve attendance statistics for a specific day.

- **Method:** `POST`
- **Path:** `/analytics/users-on-day`
- **Request Body:** `String` (ISO Date: "YYYY-MM-DD")
- **Response:** `Message<UserPresentStats>`

### Get Upcoming Birthdays
Retrieve a list of users with upcoming birthdays.

- **Method:** `GET`
- **Path:** `/analytics/upcoming-birthdays`
- **Response:** `Message<Array<UserDto>>`

### Get Attendance Rates
Retrieve overall attendance rates for different roles.

- **Method:** `GET`
- **Path:** `/analytics/attendance-rates`
- **Response:** `Message<AttendanceStats>`

### Get User Attendance History
Retrieve detailed attendance history and summary for a specific user.

- **Method:** `GET`
- **Path:** `/analytics/user-attendance/{id}`
- **Parameters:**
    - `id`: UUID
- **Response:** `Message<UserAttendanceHistory>`

---

## Data Models

### Enums

#### Role
```typescript
type Role = "Admin" | "User" | "Technical";
```

#### Hall
```typescript
type Hall = "MainHall" | "HallOne" | "Gallery" | "Basement" | "Outside";
```

#### AttendanceType
```typescript
#### ActivityType
```typescript
type ActivityType = 
    | "UserLogin" | "UserLogout" | "UserCreated" | "UserUpdated" 
    | "UserActivation" | "UserDeactivation" | "UserMarkedAttendance" 
    | "AdminMarkedAttendanceForUser" | "UserImported" | "PasswordChanged" 
    | "DeviceReset" | "EventCreated" | "EventUpdated" | "EventDeleted" 
    | "EventCheckIn" | "RosterCreated" | "RosterUpdated" | "RosterDeleted";
```

#### AttendanceType
```

#### Location `(Enum)`
```typescript
type Location = "DOA" | "CHIDA" | "OTHER";
```

### Payloads

#### LoginPayload
```typescript
interface LoginPayload {
  user: string;
  password: string;
}
```

#### CreateEventRequest
```typescript
interface CreateEventRequest {
  title: string;
  description: string;
  date: string; // "YYYY-MM-DD"
  time: string; // "HH:MM:SS"
  location: Location;
  attendance_type: string; // AttendanceType
  grace_period_in_minutes: number;
}
```

#### UpdateEventRequest
```typescript
interface UpdateEventRequest {
  event_id: string; // UUID
  title?: string;
  description?: string;
  date?: string;
  time?: string;
  location?: Location;
  attendance_type?: string;
  grace_period_in_minutes?: number;
}
```

#### CheckIntoEventRequest
```typescript
interface CheckIntoEventRequest {
  event_id: string; // UUID
  user_id: string; // UUID
  attendance_type: string;
  location?: GeoPoint;
}

#### CheckInWithIdentifierRequest
```typescript
interface CheckInWithIdentifierRequest {
  event_id: string; // UUID
  identifier: string; // Email or Reg No
  attendance_type: string;
  location?: GeoPoint;
}
```
```

#### SignAttendanceRequest
```typescript
interface SignAttendanceRequest {
  location: GeoPoint;
  device_id: string;
}
```

#### GeoPoint
```typescript
interface GeoPoint {
  lat: number;
  lng: number;
}
```

#### NewUser
```typescript
interface NewUser {
  first_name: string;
  last_name: string;
  email: string;
  password: string;
  dob?: string; // ISO DateTime
  year_joined: string;
  is_active: boolean;
  role: Role;
  gender?: string;
  phone?: string;
}
```

#### UpdateUserRequest
```typescript
interface UpdateUserRequest {
  id: string; // UUID
  first_name?: string;
  last_name?: string;
  dob?: string;
}
```

#### UserFilter
```typescript
interface UserFilter {
  page?: number;
  limit?: number;
  search?: string;
}
```

#### ChangePasswordRequest
```typescript
interface ChangePasswordRequest {
  email: string;
  password: string;
}

#### NewRoster
```typescript
interface NewRoster {
  name: string;
  is_active: boolean;
  start_date: string; // "YYYY-MM-DD"
  end_date: string; // "YYYY-MM-DD"
  num_for_hall_one: number;
  num_for_main_hall: number;
  num_for_gallery: number;
  num_for_basement: number;
  num_for_outside: number;
  year: string;
}
```

#### UpdateRosterRequest
```typescript
interface UpdateRosterRequest {
  id: string; // UUID
  name?: string;
  is_active?: boolean;
  start_date?: string;
  end_date?: string;
  num_for_hall_one?: number;
  num_for_main_hall?: number;
  num_for_gallery?: number;
  num_for_basement?: number;
  num_for_outside?: number;
  year?: string;
}

#### UpdateUserHallRequest
```typescript
interface UpdateUserHallRequest {
  user_id: string; // UUID
  user_roster_id: string; // UUID
  hall: Hall;
}
```
```

#### Pagination
```typescript
interface Pagination {
  page?: number; // default: 1
  size?: number; // default: 10
}
```
```

### Responses

#### Message<T = any>
```typescript
interface Message<T> {
  message: string;
  data: T;
}
```

#### UserDto
```typescript
interface UserDto {
  id: string; // UUID
  username?: string;
  // password hash is skipped in serialization
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
  gender?: string;
  phone?: string;
  address?: string;
  city?: string;
  state?: string;
  country?: string;
}
```

#### Event
```typescript
interface Event {
  id: string; // UUID
  title: string;
  description: string;
  date: string; // "YYYY-MM-DD"
  time: string; // "HH:MM:SS"
  grace_period_in_minutes: number;
  attendance_type: string;
  location: Location;
  created_by: string; // UUID
  created_at: string;
  updated_at: string;
}
```

#### UserAttendanceDto
```typescript
interface UserAttendanceDto {
  id: string; // UUID
  user_id: string; // UUID
  date: string; // "YYYY-MM-DD"
  week_day: string;
  time_in: string; // ISO DateTime
  time_out?: string; // ISO DateTime
  marked_by?: string; // UUID
  event_id?: string; // UUID
  attendance_type: AttendanceType;
  created_at: string;
  updated_at: string;
}
```

#### UserPresentStats
```typescript
interface UserPresentStats {
  absentees: Array<UserDto>;
  date: string; // "YYYY-MM-DD"
  presentees: Array<UserDto>;
}
```

#### AttendanceStats
```typescript
interface AttendanceStats {
  admin_rate: number;
  user_rate: number;
  technical_rate: number;
}
```

#### AttendanceSummary
```typescript
interface AttendanceSummary {
  total_days: number;
  days_present: number;
  rate: number;
}
```

#### UserAttendanceHistory
```typescript
interface UserAttendanceHistory {
  user: UserDto;
  history: Array<UserAttendanceDto>;
  summary: AttendanceSummary;
}

#### Roster
```typescript
interface Roster {
  id: string; // UUID
  name: string;
  is_active: boolean;
  start_date: string; // "YYYY-MM-DD"
  end_date: string; // "YYYY-MM-DD"
  num_for_hall_one: number;
  num_for_main_hall: number;
  num_for_gallery: number;
  num_for_basement: number;
  num_for_outside: number;
  year: string;
  created_at: string;
}
```

#### ActivityLog
```typescript
interface ActivityLog {
  id: string; // UUID
  user_id: string; // UUID
  activity_type: ActivityType;
  target_id?: string; // UUID
  target_type?: string;
  details: any;
  created_at: string;
}
```

#### ActivityLogResponse
```typescript
interface ActivityLogResponse {
  id: string; // UUID
  user_id: string; // UUID
  user_name: string;
  user_email?: string;
  user_role: string;
  first_name?: string;
  last_name?: string;
  activity_type: string;
  created_at: string;
}
```

#### PaginatedResult<T>
```typescript
interface PaginatedResult<T> {
  items: Array<T>;
  metadata: Metadata;
}
```

#### Metadata
```typescript
interface Metadata {
  page: number;
  size: number;
  total_items: number;
  num_pages: number;
}
```
```
