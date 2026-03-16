import axios from 'axios';
import { LoginCredentials, RegisterData, AuthResponse, Course, Tutor, TutorReview } from '../types';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const authAPI = {
  login: async (credentials: LoginCredentials): Promise<AuthResponse> => {
    const response = await api.post('/auth/login', credentials);
    return response.data;
  },
  
  register: async (data: RegisterData): Promise<AuthResponse> => {
    const response = await api.post('/auth/register', data);
    return response.data;
  },
};

export const courseAPI = {
  getCourses: async (): Promise<Course[]> => {
    const response = await api.get('/courses');
    return response.data;
  },
  
  getCourse: async (id: string): Promise<Course> => {
    const response = await api.get(`/courses/${id}`);
    return response.data;
  },
  
  createCourse: async (course: Partial<Course>): Promise<Course> => {
    const response = await api.post('/courses', course);
    return response.data;
  },
  
  updateCourse: async (id: string, course: Partial<Course>): Promise<Course> => {
    const response = await api.put(`/courses/${id}`, course);
    return response.data;
  },
  
  deleteCourse: async (id: string): Promise<void> => {
    await api.delete(`/courses/${id}`);
  },
  
  getMyCourses: async (): Promise<Course[]> => {
    const response = await api.get('/my/courses');
    return response.data;
  },
};

export const tutorAPI = {
  getTutors: async (): Promise<Tutor[]> => {
    const response = await api.get('/tutors');
    return response.data;
  },
  
  getTutor: async (id: string): Promise<Tutor> => {
    const response = await api.get(`/tutors/${id}`);
    return response.data;
  },
  
  searchTutors: async (specialization: string): Promise<Tutor[]> => {
    const response = await api.get(`/tutors/search?specialization=${specialization}`);
    return response.data;
  },
  
  getTutorReviews: async (id: string): Promise<TutorReview[]> => {
    const response = await api.get(`/tutors/${id}/reviews`);
    return response.data;
  },
  
  createTutorProfile: async (profile: Partial<Tutor>): Promise<Tutor> => {
    const response = await api.post('/tutors/profile', profile);
    return response.data;
  },
  
  updateTutorProfile: async (profile: Partial<Tutor>): Promise<Tutor> => {
    const response = await api.put('/tutors/profile', profile);
    return response.data;
  },
  
  addReview: async (tutorId: string, review: Partial<TutorReview>): Promise<TutorReview> => {
    const response = await api.post(`/tutors/${tutorId}/reviews`, review);
    return response.data;
  },
};

export const healthAPI = {
  check: async (): Promise<{ status: string }> => {
    const response = await api.get('/health');
    return response.data;
  },
};
