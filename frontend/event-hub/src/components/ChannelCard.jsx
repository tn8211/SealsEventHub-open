import React, { useState } from 'react';

const ChannelCard = ({ icon, title, description, details }) => {
  const [isDetailsVisible, setDetailsVisible] = useState(false);

  const handleCardClick = () => {
    setDetailsVisible(!isDetailsVisible);
  };

  return (
    <div className="w-full max-w-xs mx-auto">
      <div
        className="bg-gray-900 shadow-lg rounded-lg p-4 cursor-pointer"
        onClick={handleCardClick}
      >
        <div className="flex">
          <img src={icon} alt="Channel Icon" className="w-16 h-16 rounded-full" />
          <div className="flex-1 ml-4">
            <h3 className="text-xl font-semibold">{title}</h3>
            <p className="text-gray-300 overflow-hidden overflow-ellipsis whitespace-nowrap">
              {description.length > 30 ? description.slice(0, 30) + '...' : description}
            </p>
          </div>
        </div>
      </div>

      {/* Conditionally render event details when card is clicked */}
      {isDetailsVisible && (
        <div className="mt-4 p-4 bg-gray-800 rounded-lg text-white">
          {details}
        </div>
      )}
    </div>
  );
};

export default ChannelCard;
