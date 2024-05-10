import axios from 'axios';

// Creating an axios instance for axios to be used by the token interceptor service
const axiosInstance = axios.create({
  baseURL: `${import.meta.env.VITE_BACK_API_URL}/api`,
  headers: {
    'Content-Type': 'application/json',
  },
});

export default axiosInstance;
