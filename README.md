# Login-API

## A backend API written in Rust, that will allow a user to:
 - Login via a html form.
 - Retrieve the users To Be Read (TBR) list.
- Add a Book to the users TBR.
 - Mark a Book as Read.

## Follow Along:
- Clone the repo
- Navigate into the project directory
- Execute the initdb.sh script to set environment variables and spin up a Docker instance
    * ./scripts/initdb.sh
    * Note: If you want to skip the Docker run step, set SKIP_DOCKER=1
        SKIP_DOCKER=1 ./scripts/initdb.sh
- Cargo run

### Frameworks and Tools:

- actix-web
- rust
- tokio
- gitactions
- serde

### Milestones <br>
#### The Login handler is now saving user logon information to the database.
- Note: As web::Data\<PgConnection\> doesn't allow a mutuable reference i.e. doesn't allow multiple concurrent queries, we instead use web::Data\<PgPool\> and use a shared reference to that pool. 
 
#### Curl:
![curl](https://github.com/DaveMcMahon/Login-API/assets/16767214/39717b99-9363-4e72-a366-4e880f760222)

#### Saved Row:
<img width="933" alt="row" src="https://github.com/DaveMcMahon/Login-API/assets/16767214/8c6717f8-9479-4e9c-a3e6-a0423cf68ac7"> <br><br>

#### Integration tests using randomized ports for the http server and randomized database names by prefixing a UUID to their name in the connection string.
- Note: The DB instances created for each test run are not deleted afterwards, if performance is an issue we can add a cleanup.

#### As the project grows, this ReadMe will get updated to included relevant frameworks and tools used.
