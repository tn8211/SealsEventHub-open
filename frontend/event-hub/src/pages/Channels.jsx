import React from 'react';
import { Link } from 'react-router-dom';

const Channels = () => {
  const channels = [
    {
      title: 'Weasel 3v3 Event',
      description: 'Join us for an exciting 3v3 competition! Battle with other players and climb to the top.',
      eventId: '1',
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
    // Add more events here
  ];

  return (
    <div className="p-8 min-h-screen bg-cover bg-center" style={{ backgroundImage: "url('https://media.discordapp.net/attachments/1291400293002514442/1353156545671860247/IMG_1403.jpg?ex=67e0a0bc&is=67df4f3c&hm=82f1ba855dd1ecc79deae75a179e0b5c1dbdb969c85f5754bd258d06612e77c4&=&format=webp&width=930&height=930')" }}>
      <h1 className="text-3xl font-bold text-center mb-8 text-white">Upcoming Events</h1>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
        {channels.map((channel, index) => (
          <div key={index} className="bg-white bg-opacity-75 rounded-lg shadow-lg hover:shadow-2xl transition duration-300 ease-in-out">
            <div className="p-6">
              <h2 className="text-2xl font-semibold text-gray-800 mb-4">{channel.title}</h2>
              <p className="text-gray-600 mb-4">{channel.description}</p>
              <Link
                to={`/event/${channel.eventId}`} // Navigate to the Event Page with the eventId
                className="mt-4 inline-block px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 transition duration-300"
              >
                Show Details
              </Link>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Channels;
