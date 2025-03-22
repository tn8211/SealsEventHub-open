import React from 'react';
import { Link } from 'react-router-dom';

const Channels = () => {
  const channels = [
    {
      title: 'Event 1',
      description: 'This is event 1 description.',
      eventId: '1',  // Example ID for the event
    },
    // Add other events here
  ];

  return (
    <div className="p-8 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      {channels.map((channel, index) => (
        <div key={index} className="bg-white p-4 rounded-lg shadow-lg">
          <h2 className="text-xl font-semibold">{channel.title}</h2>
          <p className="text-gray-900">{channel.description}</p>
          <Link
            to={`/event/${channel.eventId}`} // Navigate to the Event Page with the eventId
            className="mt-4 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-700"
          >
            Show Details
          </Link>
        </div>
      ))}
    </div>
  );
};

export default Channels;
