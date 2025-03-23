import { useState } from "react";
import { Button } from "../components/Button"; // Adjust path based on your structure

const Announcements = () => {
  const [announcements, setAnnouncements] = useState([
    { id: 1, title: "System Update", content: "The system will be down for maintenance at midnight." },
    { id: 2, title: "Event Reminder", content: "Don't forget about the upcoming event this Friday!" },
  ]);
  
  const [newAnnouncement, setNewAnnouncement] = useState("");

  const postAnnouncement = () => {
    if (newAnnouncement.trim() !== "") {
      setAnnouncements([
        ...announcements,
        { id: announcements.length + 1, title: "New Announcement", content: newAnnouncement },
      ]);
      setNewAnnouncement("");
    }
  };

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">Announcements</h1>
      <div className="space-y-4">
        {announcements.map((announcement) => (
          <div
            key={announcement.id}
            className="p-4 bg-gray-900 text-white rounded-md shadow-md"
          >
            <p className="font-semibold">{announcement.title}:</p>
            <p>{announcement.content}</p>
          </div>
        ))}
      </div>
      <div className="mt-4 flex gap-2">
        <textarea
          value={newAnnouncement}
          onChange={(e) => setNewAnnouncement(e.target.value)}
          placeholder="Post a new announcement..."
          className="flex-grow p-2 border rounded-md bg-gray-800 text-white placeholder-gray-500"
        />
        <Button onClick={postAnnouncement}>Post</Button>
      </div>
    </div>
  );
};

export default Announcements;
