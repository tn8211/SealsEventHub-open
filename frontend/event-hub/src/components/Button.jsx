// src/components/ui/button.jsx
export const Button = ({ onClick, children }) => (
  <button
    onClick={onClick}
    className="bg-blue-500 text-white p-2 rounded-md hover:bg-blue-700"
  >
    {children}
  </button>
);
