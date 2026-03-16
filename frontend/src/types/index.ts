export interface User {
  id: string;
  email: string;
  role: 'student' | 'tutor' | 'admin';
  created_at: string;
}

export interface Tutor {
  id: string;
  user_id: string;
  bio: string;
  specializations: string[];
  hourly_rate: number;
  rating: number;
  created_at: string;
  updated_at: string;
  user?: User;
}

export interface Course {
  id: string;
  tutor_id: string;
  title: string;
  description: string;
  price: number;
  difficulty_level: 'beginner' | 'intermediate' | 'advanced';
  is_active: boolean;
  created_at: string;
  updated_at: string;
  tutor?: Tutor;
}

export interface TutorReview {
  id: string;
  tutor_id: string;
  student_id: string;
  rating: number;
  comment: string;
  created_at: string;
  student?: User;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterData {
  email: string;
  password: string;
  role: 'student' | 'tutor';
}
