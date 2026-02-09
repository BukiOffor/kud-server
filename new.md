# New API Endpoints Documentation

This document provides a detailed overview of the newly added API endpoints and their data structures.

---

## 🔐 Users & Authentication

### Reset User Device ID
Allows an administrator to reset a user's registered device ID. This is useful when a user changes their device and needs to log in from a new one.

- **Method:** `PATCH`
- **Path:** `/api/v1/users/admin/reset-device-id/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): The UUID of the user whose device ID should be reset.
- **Response:** `Message<()>`
  - **Example:**
    ```json
    {
      "message": "User device ID reset successfully",
      "data": null
    }
    ```

---

## 📜 Activity Logs 🔍

### Get All Activity Logs (Paginated)
Retrieves a paginated list of all system activity logs. This is used for auditing and tracking state changes across the application.

- **Method:** `GET`
- **Path:** `/api/v1/logs`
- **Permissions:** Admin Only
- **Query Parameters:**
  - `page`: i32 (default: 1)
  - `size`: i32 (default: 10)
- **Response:** `PaginatedResult<ActivityLogResponse>`
  - **Example:**
    ```json
    {
      "items": [
        {
          "id": "018d8e12-...",
          "user_id": "018d8e10-...",
          "user_name": "John Doe",
          "user_role": "Admin",
          "activity_type": "UserLogin",
          "created_at": "2024-02-09T15:22:57"
        }
      ],
      "metadata": {
        "page": 1,
        "size": 10,
        "total_items": 150,
        "num_pages": 15
      }
    }
    ```

### Get User-Specific Activity
Retrieves all logs associated with a specific user.

- **Method:** `GET`
- **Path:** `/api/v1/logs/user/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): User UUID.
- **Response:** `Array<ActivityLog>`

---

## 📋 Roster Management


### Activate Roster
Activates a roster and automatically allocates users to halls based on their past allocations and current capacity.

- **Method:** `PATCH`
- **Path:** `/api/v1/roster/activate/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): UUID of the roster to activate.
- **Response:** `Message<()>`

### Delete Roster
Deletes a roster and its associated hall allocations.

- **Method:** `DELETE`
- **Path:** `/api/v1/roster/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): UUID of the roster to delete.
- **Response:** `Message<()>`

### View Roster Assignments
Retrieves all users assigned to a specific roster along with their hall allocations.

- **Method:** `GET`
- **Path:** `/api/v1/roster/{id}/assignments`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): UUID of the roster.
- **Response:** `Array<RosterAssignmentDto>`

### Export Roster (CSV)
Generates and downloads a CSV file containing all user assignments for a specific roster.

- **Method:** `GET`
- **Path:** `/api/v1/roster/export/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): UUID of the roster to export.
- **Response:** `File (text/csv)`
  - **Filename:** `roster_{roster_name}.csv`

### Export Roster by Hall (CSV)
Generates and downloads a CSV file for a specific hall within a roster.

- **Method:** `GET`
- **Path:** `/api/v1/roster/export/{id}/hall`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): UUID of the roster.
- **Query Parameters:**
  - `hall`: `Hall` (MainHall, HallOne, Gallery, Basement, Outside)
- **Response:** `File (text/csv)`

---

## � Analytics & Reports

### Get Total Users List
Retrieves a complete list of users for analytics purposes.

- **Method:** `GET`
- **Path:** `/api/v1/analytics/total-users`
- **Permissions:** Admin Only
- **Response:** `Message<Array<UserDto>>`

### Get Users Present on Day
Calculates stats and fetches users present on a specific date.

- **Method:** `GET`
- **Path:** `/api/v1/analytics/users-on-day`
- **Permissions:** Admin Only
- **Query Parameters:**
  - `date`: string (YYYY-MM-DD)
- **Response:** `Message<UserPresentStats>`

### Get Overall Attendance Rates
Retrieves attendance percentage stats aggregated by user role.

- **Method:** `GET`
- **Path:** `/api/v1/analytics/attendance-rates`
- **Permissions:** Admin Only
- **Response:** `Message<AttendanceStats>`

### Get User Attendance History
Retrieves detailed attendance history and summary for a specific user.

- **Method:** `GET`
- **Path:** `/api/v1/analytics/user-attendance/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): User UUID.
- **Response:** `Message<UserAttendanceHistory>`

### Upcoming Birthdays
Retrieves a list of users with birthdays in the next 30 days.

- **Method:** `GET`
- **Path:** `/api/v1/analytics/upcoming-birthdays`
- **Permissions:** Admin Only
- **Response:** `Message<Array<UserDto>>`

### Event Stats Report
Retrieves detailed statistics and attendance breakdown for a specific event.

- **Method:** `GET`
- **Path:** `/api/v1/analytics/event-report/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): Event UUID.
- **Response:** `Message<EventStatsReport>`

---

## �👥 Attendance Tracking

### Check-in Attendance
Allows a user to mark their own attendance for an ongoing event.

- **Method:** `POST`
- **Path:** `/api/v1/attendance/check-in`
- **Permissions:** Authenticated User
- **Request Body:** `SignAttendanceRequest`
- **Response:** `Message<()>`

### Admin Sign Attendance
Allows an administrator to mark attendance for a specific user.

- **Method:** `GET`
- **Path:** `/api/v1/attendance/admin/sign/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): The UUID of the user to mark attendance for.
- **Response:** `Message<()>`

### Get Attendance for a Specific Day
Retrieves all attendance records for a given date, including detailed user information for each record.

- **Method:** `GET`
- **Path:** `/api/v1/attendance/on-day/{date}`
- **Permissions:** Authenticated User
- **Parameters:**
  - `date` (Path): string in `YYYY-MM-DD` format.
- **Response:** `Message<Array<AttendanceWithUser>>`

### Revoke User Attendance
Allows an administrator to delete/revoke an attendance record.

- **Method:** `DELETE`
- **Path:** `/api/v1/attendance/admin/revoke/{id}`
- **Permissions:** Admin Only
- **Parameters:**
  - `id` (Path): The UUID of the attendance record to revoke.
- **Response:** `Message<()>`

---

## 🏗️ Data Models (DTOs)

### Common
#### Message<T>
```typescript
interface Message<T> {
  message: string;
  data: T | null;
}
```

#### PaginatedResult<T>
```typescript
interface PaginatedResult<T> {
  items: T[];
  metadata: {
    page: number;
    size: number;
    total_items: number;
    num_pages: number;
  };
}
```

### Attendance
#### AttendanceWithUser
```typescript
interface AttendanceWithUser {
  attendance: UserAttendanceDto;
  user: UserDto;
}
```

#### UserAttendanceDto
```typescript
interface UserAttendanceDto {
  id: string; // UUID
  user_id: string; // UUID
  date: string; // "YYYY-MM-DD"
  time_in: string; // ISO 8601
  time_out: string | null;
  marked_by: string | null; // UUID
  event_id: string | null; // UUID
  attendance_type: "Remote" | "Onsite" | "Mandatory" | "Optional" | "Standard" | "Late" | "Excused";
  created_at: string;
  updated_at: string;
  week_day: string;
}
```

### Roster
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

#### RosterAssignmentDto
```typescript
interface RosterAssignmentDto {
  id: string; // UUID of the assignment record
  user_id: string; // UUID of the user
  first_name: string;
  last_name: string;
  reg_no: string;
  hall: "MainHall" | "HallOne" | "Gallery" | "Basement" | "Outside";
}
```
```

### Activity Logs
#### ActivityLog
```typescript
interface ActivityLog {
  id: string;
  user_id: string;
  activity_type: ActivityType;
  target_id: string | null;
  target_type: string | null;
  details: any;
  created_at: string;
}
```

#### ActivityLogResponse
```typescript
interface ActivityLogResponse {
  id: string;
  user_id: string;
  user_name: string;
  user_email: string | null;
  user_role: string;
  first_name: string | null;
  last_name: string | null;
  activity_type: string;
  created_at: string;
}

### Analytics
#### UserPresentStats
```typescript
interface UserPresentStats {
  absentees: UserDto[];
  date: string; // "YYYY-MM-DD"
  presentees: UserDto[];
}
```

#### AttendanceStats
```typescript
interface AttendanceStats {
  admin_rate: number;
  user_rate: number;
  technical_rate: number;
  total_users: number;
  active_users: number;
  suspended_users: number;
}
```

#### UserAttendanceHistory
```typescript
interface UserAttendanceHistory {
  user: UserDto;
  history: UserAttendanceDto[];
  summary: {
    total_days: number;
    days_present: number;
    rate: number;
  };
}
```

#### EventStatsReport
```typescript
interface EventStatsReport {
  total_attendees: number;
  eligible_attendees_count: number;
  attendees: {
    user_id: string;
    first_name: string;
    last_name: string;
    email: string;
    time_in: string; // ISO 8601 or HH:mm
  }[];
  absentees: UserDto[];
}
```
```

#### ActivityType (Enum)
`"UserLogin" | "UserLogout" | "UserCreated" | "UserUpdated" | "UserActivation" | "UserDeactivation" | "UserMarkedAttendance" | "AdminMarkedAttendanceForUser" | "UserImported" | "PasswordChanged" | "DeviceReset" | "EventCreated" | "EventUpdated" | "EventDeleted" | "EventCheckIn" | "RosterCreated" | "RosterUpdated" | "RosterDeleted" | "AttendanceRevoked"`