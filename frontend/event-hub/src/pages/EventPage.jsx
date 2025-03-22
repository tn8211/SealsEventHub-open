import React from 'react';
import { useParams, useNavigate } from 'react-router-dom';

const EventPage = () => {
  const { eventId } = useParams();  // Get eventId from the URL
  const navigate = useNavigate();

  // Example data (replace this with your dynamic data if you have it)
  const events = [
    {
      id: '1',
      title: 'WEASEL POWER PLAY (3V3 TOURNAMENT)',
      description: 'This is the event description for the 3v3 Tournament.',
      prize: '3 BP+ for the winning team!',
    },
    // Add more events here as needed
  ];

  // Find the event based on eventId
  const event = events.find((e) => e.id === eventId);

  // If no event is found, show a message
  if (!event) {
    return <p>Event not found.</p>;
  }

  return (
    <div className="container mx-auto p-6 bg-gray-800 text-white rounded-lg shadow-lg">
      {/* Back Button */}
      <button
        className="text-blue-500 hover:text-blue-700 mb-4"
        onClick={() => navigate(-1)} // Navigate back to the previous page
      >
        &larr; Back
      </button>

      <h1 className="text-3xl font-semibold text-center mb-4">🔥 {event.title} 🔥</h1>
      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">🏆 Prize:</h3>
        <p>{event.prize}</p>
      </div>

      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">📌 Details:</h3>
        <p>{event.description}</p>
      </div>

      {/* Additional event details can be added here */}

      <div className="text-center mt-6">
        <p className="text-2xl font-bold text-red-500">MAY THE BEST TEAM WIN!! 🔥</p>
      </div>
    </div>
  );
};

export default EventPage;
