import React from 'react';
import { useState } from 'react';
import './index.css';
import Navbar from './components/Navbar';
import Homepage from './pages/Homepage';
import Channels from './pages/Channels';
import Teams from './pages/Teams';
import Inbox from './pages/Inbox';
import { BrowserRouter, Routes, Route} from 'react-router-dom';
import PopupManager from './components/PopupManager';
import LandingPage from './pages/LandingPage';
import { AuthProvider } from './components/Context/AuthContext';

const App = () => {

  const [activePopup, setActivePopup] = useState(null);

  const openPopup = (popupType) => {
    setActivePopup(popupType);
  };

  const closePopup = () => {
    setActivePopup(null);
  };

  return (
    <AuthProvider>
      <BrowserRouter>
        <Navbar openPopup={openPopup}/>
        <div>
        {activePopup && <PopupManager activePopup={activePopup} closePopup={closePopup} />}
          <Routes>
            <Route path="/" element={<LandingPage />} />
            <Route path="/homepage" element={<Homepage />} />
            <Route path="/channels" element={<Channels />} />
            <Route path="/teams" element={<Teams />} />
            <Route path="/inbox" element={<Inbox />} />
          </Routes>
        </div>
      </BrowserRouter>
    </AuthProvider>
  );
};

export default App;
