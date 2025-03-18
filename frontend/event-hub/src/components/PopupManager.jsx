import React from 'react';
import { X } from 'lucide-react';
import LoginForm from './Popups/LoginForm';
import SignUpForm from './Popups/SignUpForm';
import ProfilePopup from './Popups/ProfilePopup';

const PopupManager = ({ activePopup, closePopup }) => {
  const popupComponents = {
    signup: <SignUpForm closePopup={closePopup} />,
    signin: <LoginForm closePopup={closePopup} />,
    profile: <ProfilePopup closePopup={closePopup} />,
  };

  return (
    <div className="fixed inset-0 z-50 flex justify-center items-center bg-gray-600 bg-opacity-20">
      <div className="bg-gray-800 rounded-lg p-6 w-96 relative">
        <button
          onClick={closePopup}
          className="absolute top-2 right-2 rounded-lg text-white text-xl font-bold hover:bg-red-700"
        >
          <X />
        </button>
        {popupComponents[activePopup] || <div>Invalid pop-up type</div>}
      </div>
    </div>
  );
};

export default PopupManager;
