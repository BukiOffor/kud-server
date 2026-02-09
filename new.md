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

### Create New Roster
Creates a new roster for church hall allocations.

- **Method:** `POST`
- **Path:** `/api/v1/roster/create`
- **Permissions:** Admin Only
- **Request Body:** `NewRoster`
- **Response:** `Message<Roster>`

### Update Existing Roster
Updates details of an existing roster. Supports partial updates.

- **Method:** `PATCH`
- **Path:** `/api/v1/roster/update`
- **Permissions:** Admin Only
- **Request Body:** `UpdateRosterRequest`
- **Response:** `Message<Roster>`

### Get Roster by ID
- **Method:** `GET`
- **Path:** `/api/v1/roster/{id}`
- **Response:** `Roster`

### Get All Rosters
- **Method:** `GET`
- **Path:** `/api/v1/roster/all`
- **Response:** `Array<Roster>`

---

## 👥 Attendance Tracking

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
```

#### ActivityType (Enum)
`"UserLogin" | "UserLogout" | "UserCreated" | "UserUpdated" | "UserActivation" | "UserDeactivation" | "UserMarkedAttendance" | "AdminMarkedAttendanceForUser" | "UserImported" | "PasswordChanged" | "DeviceReset" | "EventCreated" | "EventUpdated" | "EventDeleted" | "EventCheckIn" | "RosterCreated" | "RosterUpdated" | "RosterDeleted" | "AttendanceRevoked"`