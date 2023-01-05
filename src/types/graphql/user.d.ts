export interface UsersResponse {
  users: User[]
}

export interface User {
  id: string
  email: string
  name: string
  role: Role
  status: Status
}

export type Role = "STUDENT" | "TEACHER" | "GUEST" | "ADMIN"
export type Status = "OFFLINE" | "ONLINE" | "HIDDEN"
