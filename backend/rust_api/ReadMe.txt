To get this to run you need to create a certificare fot TLS. Add a certificate and the key to this directory as "cert.pem" and "ukey.pem" respectively

Also you need to add a .env file to the "src" directory that contains:
DATABASE_URL=<your connection string>
JWT_SECRET=<your key for json web tokens>

the default connection string used for the environment provided by this repository is:
DATABASE_URL=postgres://root:groot@database:5432/postgres_db

