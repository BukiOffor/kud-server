### Reset Device ID
Reset a user's device ID (Admin only).

- **Method:** `PATCH`
- **Path:** `/users/admin/reset-device-id/{id}`
- **Parameters:**
    - `id`: UUID
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
---




## Attendance (`/attendance`)

### Get Attendance on Day
Retrieve all attendance records for a specific day.

- **Method:** `GET`
- **Path:** `/attendance/on-day/{date}`
- **Parameters:**
    - `date`: string (YYYY-MM-DD)
- **Response:** `Message<Array<AttendanceWithUser>>`

### Revoke Attendance
Delete an attendance record (Admin only).

- **Method:** `DELETE`
- **Path:** `/attendance/admin/revoke/{id}`
- **Parameters:**
    - `id`: UUID
- **Response:** `Message`

---


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

#### Pagination
```typescript
interface Pagination {
  page?: number; // default: 1
  size?: number; // default: 10
}
```




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