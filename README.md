# Source code for a webapp to handle events

---

### What is this project?
This is a simple webapp meant for simplifying event hosting for small communities.  
**This Project is still in developement**: it has been published already to share progress with the community I'm building this for.
The frontend is made with React and TailwindCSS, while the backend is written with Rust using the Rocket framework.

### What will the final version offer?
The goal for this webapp is to let users create channels they can use to post events in. Other users will be able to subscribe to specific channels to see and sign up for events. Users will also be able to create and manage teams for joining tournaments. A permission system will be in place to restrict access to certain operations such as editing events.

### What is the scope of this project?
This is just a project made for fun by a computer science student so don't expect anything incredible :\)

# Build instructions

---

### Backend
To build the backend you need to have [Docker](www.docker.com) installed.  
Then you need to create a TLS certificate and save it as `/backend/rust_api/cert.pem`  
Save the private key as `/backend/rust_api/ukey.pem`

Once that's done just use the `docker compose up` command to start it. This will automatically build the API, start the Postgresqldatabase and start pgadmin to interact with the database through a GUI.

**IMPORTANT**: make sure you change the JWT_SECRET value in /backend/rust_api/src/.env from the default one provided here as that is the key that will be used for encryption of the Tokens

###Frontend
The frontend requires [Node.js](https://nodejs.org). Once installed, navigate to `/frontend/event-hub`  
and run `npm i` to install all the necessary dependencies. At that point you can run the project locally with  
`npm run dev` or on your local network with `npm run dev -- --host`  
  
Again, this is not functional yet so keep it in mind :\)