import React from 'react';
import { useNavigate } from 'react-router-dom';

const Channels = () => {
  const navigate = useNavigate();

  // Define an array of event objects
  const channels = [
    {
      title: 'Weasel 3v3 Event', // Title of the event
      description: 'Join us for an exciting 3v3 competition! Battle with other players and climb to the top.', // Short description
      eventId: '1', // Unique identifier for routing
    },
    {
      title: 'Tournament of Champions',
      description: 'Compete in the grand tournament with prizes for the top players.',
      eventId: '2',
    },
    {
      title: 'Weekend Warzone',
      description: 'Gear up for the weekend warzone event with intense battles and rewards!',
      eventId: '3',
    },
  ];

  return (
    <div
      className="p-8 min-h-screen bg-cover bg-center"
      style={{
        backgroundImage:
          "url('https://media.discordapp.net/attachments/1291400293002514442/1353156545671860247/IMG_1403.jpg?ex=67e0a0bc&is=67df4f3c&hm=82f1ba855dd1ecc79deae75a179e0b5c1dbdb969c85f5754bd258d06612e77c4&=&format=webp&width=930&height=930')",
      }}
    >
      <h1 className="text-3xl font-bold text-center mb-8 text-black">Upcoming Events</h1>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
        {channels.map((channel, index) => (
          <div
            key={index}
            className="bg-white bg-opacity-75 rounded-lg shadow-lg hover:shadow-2xl transition duration-300 ease-in-out flex flex-col"
          >
            <div className="p-6 flex-1">
              {/* Making the title more bold */}
              <h2 className="text-2xl font-bold text-gray-800 mb-4">{channel.title}</h2>
              {/* Making the description less bold */}
              <p className="text-xl font-medium text-black mb-4">{channel.description}</p>
            </div>

            {/* Button container to ensure uniform alignment */}
            <div className="flex justify-center mt-auto mb-4">
              <button
                onClick={() => navigate(`/event/${channel.eventId}`)} // Navigate to event details
                className="inline-block px-4 py-2 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 hover:scale-105 hover:shadow-lg transform transition-all duration-300 ease-in-out"
                style={{
                  height: '40px', // Same height for all buttons
                  width: '370px', // Shortened width for buttons
                  marginTop: '-10px', // Move button slightly higher within the container
                }}
              >
                Show Details
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Channels;
