import React from 'react';
import { Turtle, Menu, X, User } from 'lucide-react';
import { useState } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from './Context/AuthContext';

let navigationItems = [
    {label: "Home", href: "/homepage"},
    {label: "Channels", href: "/channels"},
    {label: "Teams", href: "/teams"},
    {label: "Inbox", href: "/inbox"},
]

const Navbar = ({openPopup}) => {

    const [mobileDrawerOpen, setmobileDrawerOpen] = useState(false);
    const [selectedTab, setSelectedTab] = useState("");

    const { user } = useAuth();

    const toggleNavbar = () => {
        setmobileDrawerOpen(!mobileDrawerOpen);
    }

    function profileButton(classes) {
        return (
            <div className={classes}>
                <button 
                    onClick={() => openPopup('profile')}
                    className="text-white bg-gradient-to-r from-orange-500 to-orange-800 hover:from-orange-600 hover:to-orange-900 px-4 py-2 rounded-md flex items-center"
                >
                    {user.username}
                    <User className="ml-2" />
                </button>
            </div>
        )
    }

    function loginAndSignupButtons(classes) {
        return (
            <div className={classes}>
                <button
                    onClick={() => openPopup('signin')}
                    className="text-white hover:bg-gray-700 px-4 py-2 rounded-md"
                >
                        Sign In
                </button>
                <button
                    onClick={() => openPopup('signup')}
                    className="text-white bg-gradient-to-r from-orange-500 to-orange-800 hover:from-orange-600 hover:to-orange-900 px-4 py-2 rounded-md"
                >
                        Sign Up
                </button>
            </div>
        )
    }

  return (
    <nav className="sticky top-0 z-50 py-3 border-b border-neutral-700/80 bg-gray-900">
        <div className="container px-4 relative text-sm">
            <div className="flex justify-between items-center w-screen">
                <div className="flex items-center flex-shrink-0">
                    <Turtle />
                    <span className="text-xl tracking-tight">Seal's Event Hub</span>
                </div>
                <ul className="hidden lg:flex ml-14 space-x-6">
                    {navigationItems.map((item, index) => (
                        <li key={index}>
                            <div className={`flex px-2 py-2 ${selectedTab == item.label ? "border-b-2 border-b-orange-500" : ""}`}>
                                <Link to={item.href} onClick={ () => {setSelectedTab(item.label)}}>{item.label}</Link>
                            </div>
                        </li>
                    ))}
                </ul>
                {(user) ? profileButton("hidden lg:flex mx-5 justify-center space-x-2 items-center") : loginAndSignupButtons("hidden lg:flex mx-5 justify-center space-x-6 items-center")}
                <div className="lg:hidden md:flex mx-8 flex-col justify-end">
                    <button onClick={toggleNavbar}>
                        {mobileDrawerOpen ? <X /> : <Menu />}
                    </button>
                </div>
            </div>
            {
                <div className={`fixed right-0 z-20 my-3 bg-neutral-900 w-full p-12 flex flex-col justify-center items-center lg:hidden
                    transition-all duration-300 ease-in-out
                    ${mobileDrawerOpen ? "-translate-y-0 opacity-100" : "-translate-y-full opacity-0 pointer-events-none"}
                `}>
                    <ul>
                        {navigationItems.map((item, index) => (
                            <li key={index} className="py-4">
                                <div className={`px-2 py-2 text-center ${selectedTab == item.label ? "border-b-2 border-b-orange-500" : ""}`}>
                                    <Link to={item.href} onClick={ () => {
                                        setSelectedTab(item.label);
                                        toggleNavbar();
                                        }}>{item.label}</Link>
                                </div>
                            </li>
                        ))}
                    </ul>
                
                {(user) ? profileButton("mt-5 flex space-x-2 items-center") : loginAndSignupButtons("mt-5 flex space-x-6") }
                
                </div>
            }
        </div>
    </nav>
  )
}

export default Navbar