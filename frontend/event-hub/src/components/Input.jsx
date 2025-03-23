// src/components/ui/input.jsx
export const Input = ({ value, onChange, placeholder, className }) => (
  <input
    type="text"
    value={value}
    onChange={onChange}
    placeholder={placeholder}
    className={`border p-2 rounded-md ${className}`}
  />
);
