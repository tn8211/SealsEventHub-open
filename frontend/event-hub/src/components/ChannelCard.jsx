import React, { useState } from 'react';

const ChannelCard = ({ icon, title, description, details }) => {
  const [isDetailsVisible, setDetailsVisible] = useState(false);

  const handleCardClick = () => {
    setDetailsVisible(!isDetailsVisible);
  };

  return (
    <div className="w-full max-w-xs mx-auto">
      {/* Main Card */}
      <div
        className="bg-gray-900 hover:bg-gray-900 shadow-lg rounded-lg p-4 cursor-pointer transition duration-300"
        onClick={handleCardClick}
      >
        <div className="flex items-center">
          {/* Channel Icon */}
          <img
            src={icon}
            alt="Channel Icon"
            className="w-12 h-12 rounded-full mr-4 border-2 border-white"
          />
          <div className="flex-1">
            {/* Channel Title */}
            <h3 className="text-white font-semibold text-lg">{title}</h3>
            {/* Channel Description */}
            <p className="text-gray-300 text-sm overflow-hidden overflow-ellipsis whitespace-nowrap">
              {description.length > 30 ? description.slice(0, 30) + '...' : description}
            </p>
          </div>
        </div>
      </div>

      {/* Event Details */}
      {isDetailsVisible && (
        <div className="mt-2 p-4 bg-gray-700 rounded-lg text-white">
          <h4 className="text-md font-semibold">Weasel 3v3 Event</h4>
          <p className="text-gray-300 text-sm">{details}</p>
        </div>
      )}
    </div>
  );
};

export default ChannelCard;
