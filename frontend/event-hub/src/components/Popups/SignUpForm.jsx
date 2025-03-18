import React from "react";
import { useState } from 'react';
import AuthForm from "../AbstractComponents/AuthForm";

const SignUpForm = ({closePopup}) => {

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');

    const handleSubmit = async (event) => {
        event.preventDefault();
    
        // Validation
        if (!username || !password) {
          setError('Please enter both username and password.');
          return;
        }
    
        try {
          const response = await fetch(`${import.meta.env.VITE_SERVER_BASE_URL}/signup`, {
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
            throw new Error(data.message || 'Signup failed');
          }

          alert("Signup successful!");
          closePopup();
          
        } catch (error) {
          console.error('Error during login:', error);
          setError('Login failed. Please try again.');
        }
      };

    return (
        <AuthForm 
            isSignup={true} 
            handleSubmit={handleSubmit} 
            username={username} 
            password={password} 
            setUsername={setUsername} 
            setPassword={setPassword} 
            error={error}
        />
    )
}

export default SignUpForm