const AuthForm = ({isSignup, handleSubmit, username, password, setUsername, setPassword, error}) => {

  return (
    <div className="w-full max-w-sm mx-auto px-6 py-8 bg-gray-800 rounded-lg shadow-lg">
      <h2 className="text-2xl text-white text-center mb-6">
        {isSignup ? 'Sign Up' : 'Login'}
      </h2>

      {error && <p className="text-red-500 text-center">{error}</p>}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label htmlFor="username" className="block text-sm text-white">Username</label>
          <input
            type="text"
            id="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
            className="w-full mt-1 px-4 py-2 bg-gray-700 text-white border border-gray-600 rounded-md focus:ring-2 focus:ring-orange-500"
          />
        </div>
        <div>
          <label htmlFor="password" className="block text-sm text-white">Password</label>
          <input
            type="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            className="w-full mt-1 px-4 py-2 bg-gray-700 text-white border border-gray-600 rounded-md focus:ring-2 focus:ring-orange-500"
          />
        </div>
        <button
          type="submit"
          className="w-full mt-4 py-2 px-4 bg-orange-500 text-white rounded-md hover:bg-orange-600 transition"
        >
          {isSignup ? 'Sign Up' : 'Login'}
        </button>
      </form>
    </div>
  );
};

export default AuthForm;