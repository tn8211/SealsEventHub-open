import React from "react";
import { useAuth } from "../Context/AuthContext";
import { useNavigate } from "react-router-dom";
import { User } from "lucide-react";

const ProfilePopup = ({closePopup}) => {

    const navigate = useNavigate();
    const { logout } = useAuth();

    return (
        <div className="w-full max-w-sm mx-auto flex flex-col justify-center items-center px-6 py-8 bg-gray-800 rounded-lg shadow-lg">
            <User />
            <h2 className="text-center">
                Cool user infos that are not implemented yet
            </h2>
            <button className="px-4 py-2 mt-3 text-black bg-teal-500 hover:bg-teal-700 rounded-md"
            onClick={() => {
                logout();
                closePopup();
                navigate("/");
            }}>
                Logout
            </button>
        </div>
    )
}

export default ProfilePopup