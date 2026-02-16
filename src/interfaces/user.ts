export interface User {
    avatar_url: string | null,
    bio: string | null,
    birthday: string | null,
    created_at: string
    display_name: string | null,
    email: string,
    gender: string | null,
    id: string,
    locale: string | null,
    name: string | null,
    phone: string | null,
    preferences: Record<string, any> | null,
    role: string | null,
    surname: string | null,
    timezone: string | null,
    username: string
}