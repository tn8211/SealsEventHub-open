import React from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';

const EventPage = () => {
  const { eventId } = useParams(); // Get eventId from the URL
  const navigate = useNavigate();

  // Example data (replace this with your dynamic data if you have it)
  const events = [
    {
      id: '1',
      title: 'WEASEL POWER PLAY (3V3 TOURNAMENT)',
      description: 'This is the event description for the 3v3 Tournament.',
      prize: '3 BP+ for the winning team!',
      fullMessage: `WEASEL POWER PLAY (3V3 TOURNAMENT)\n
🔥 Competitive 3v3 Showdown | 2~3 Weeks Duration\n
🏆 Prize: 3 BP+ for the winning team!(sponsored by King DaviTo)\n\n
📌 TEAM FORMATION:\n
3 players + 1 sub (optional)\n
Teams with at least 2 weasel members (sub doesn't count) can register before non-weasel teams\n
After third day of registration, anyone in the server can participate\n\n
Open to all regions, but regional teams preferred.\n
Prohibited from playing on secondary/different accounts.\n
Register by posting in tournament-registration.\n
(Registration starts at Friday, March 21, 2025 2:00 AM and Ends at Sunday, March 30, 2025 1:00 AM)\n
Substitutes must be declared during registration publicly or in tournament mod DMs.\n\n
🏆 TOURNAMENT FORMAT:\n
Round 1 – Swiss System: 2-4 rounds, no eliminations, fair matchups. (No 2 teams will face each other more than once)\n
Round 2 – Direct Elimination: Top teams battle for the championship!\n\n
📊 Live Rankings & Match Results: (https://challonge.com/x5gtqcpz)\n\n
⚔ MATCH RULES:\n
Rounds: Approx. 72 hours per round + rest day between.\n
Draft Games: Best of 3 per round (Finals/Semis: Best of 5).\n
Maps: Randomly drawn from the provided document.\n
Result Submission: Post in score-report. Cheating = Punishment.\n\n
📅 SCHEDULING & CONDUCT:\n
Teams schedule their own matches or get assigned a time on the break day.\n
No-shows = Forfeit = Points to opposing team.\n\n
Sportsmanship is key – misconduct will be penalised!\n\n
MAY THE BEST TEAM WIN!! 🔥`,
    },
  ];

  // Find the event based on eventId
  const event = events.find((e) => e.id === eventId);

  // If no event is found, show a message
  if (!event) {
    return <p>Event not found.</p>;
  }

  return (
    <motion.div
      className="relative p-8 min-h-screen"
      style={{
        backgroundImage:
          "url('https://media.discordapp.net/attachments/1291400293002514442/1353198369694290003/Untitled_design_1.png?ex=67e0c7b0&is=67df7630&hm=48396794d8672d28fd6094ce0ff0eee172df513de494a9b4d1ff927fa3f384cf&=&format=webp&quality=lossless&width=930&height=930')",
        backgroundPosition: 'center',
        backgroundSize: 'cover',
      }}
      initial={{ opacity: 0, y: 50 }} // Start position off-screen and with 0 opacity
      animate={{ opacity: 1, y: 0 }}  // Fade in and move up to original position
      transition={{
        duration: 1.5, // Longer duration for a smoother transition
        ease: [0.25, 0.8, 0.25, 1], // Ease function for a more natural motion
      }} 
    >
      {/* Full-screen background with strong blur */}
      <motion.div
        className="absolute top-0 left-0 right-0 bottom-0"
        style={{
          backgroundColor: 'rgba(0, 0, 0, 0.5)', // Darken the background
          backdropFilter: 'blur(15px)', // Apply heavy blur
          zIndex: -1, // Put behind the content
          opacity: 0,
          transition: 'opacity 1.5s ease-out', // Smooth opacity transition
          animate: { opacity: 1 }, // Gradual opacity increase
        }}
      ></motion.div>

      {/* Back Button with Cute Animation */}
      <motion.button
        className="text-blue-500 hover:text-blue-700 mb-4"
        onClick={() => navigate(-1)} // Navigate back to the previous page
        whileHover={{ scale: 1.1, rotate: 10 }}
        transition={{ duration: 0.4 }} // Longer hover duration for smoother effect
      >
        &larr; Back
      </motion.button>

      {/* Title and Event Info */}
      <h1
        className="text-5xl font-semibold text-center mb-4 text-white"
        style={{
          WebkitTextFillColor: 'black', // Black text fill color
        }}
      >
        {event.title}
      </h1>

      <div className="mb-6">
        <h3
          className="text-3xl font-semibold mb-2 text-white"
          style={{
            WebkitTextFillColor: 'black', // Black text fill color
          }}
        >
          🏆 Prize:
        </h3>
        <p
          className="text-white text-xl"
          style={{
            WebkitTextFillColor: 'black', // Black text fill color
          }}
        >
          {event.prize}
        </p>
      </div>

      <div className="mb-6">
        <h3
          className="text-3xl font-semibold mb-2 text-white"
          style={{
            WebkitTextFillColor: 'black', // Black text fill color
          }}
        >
          📌 Details:
        </h3>
        <p
          className="text-white text-xl whitespace-pre-line"
          style={{
            WebkitTextFillColor: 'black', // Black text fill color
          }}
        >
          {event.fullMessage}
        </p>
      </div>

      {/* Additional event details can be added here */}
      <div className="text-center mt-6">
        <p
          className="text-4xl font-bold text-red-500"
          style={{
            WebkitTextFillColor: 'black', // Black text fill color
          }}
        >
          MAY THE BEST TEAM WIN!! 🔥
        </p>
      </div>
    </motion.div>
  );
};

export default EventPage;
