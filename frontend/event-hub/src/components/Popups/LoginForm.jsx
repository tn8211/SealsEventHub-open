import React from "react";
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import AuthForm from "../AbstractComponents/AuthForm";
import { useAuth } from "../Context/AuthContext";

const LoginForm = ({closePopup}) => {

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    
    const navigate = useNavigate();
    const { login } = useAuth();

    const handleSubmit = async (event) => {
        event.preventDefault();
    
        // Validation
        if (!username || !password) {
          setError('Please enter both username and password.');
          return;
        }
    
        try {
          const response = await fetch(`${import.meta.env.VITE_SERVER_BASE_URL}/login`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({
              username,
              password,
            }),
          });
    
          const data = await response.json();
    
          if (!response.ok) {
            throw new Error(data.message || 'Login failed');
          }

          localStorage.setItem('authToken', data.token);
          login({ username: username, token: data.token });
          navigate("/homepage");
          closePopup();
          
        } catch (error) {
          console.error('Error during login:', error);
          setError('Login failed. Please try again.');
        }
      };

    return (
        <AuthForm 
            isSignup={false} 
            handleSubmit={handleSubmit} 
            username={username} 
            password={password} 
            setUsername={setUsername} 
            setPassword={setPassword} 
            error={error}
        />
    )
}

export default LoginForm 