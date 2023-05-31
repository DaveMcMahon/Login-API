# Login-API

## A backend API written in Rust, that will allow a user to:
 - Login via a html form.
 - Retrieve the users To Be Read (TBR) list.
- Add a Book to the users TBR.
 - Mark a Book as Read.

### Frameworks and Tools:

- actix-web
- rust
- tokio
- gitactions
- serde

Sending a correctly formatted payload to the login endpoint successfully connects to the database and inserts the fields into the Logins table. As web::Data<PgConnection> doesn't allow a mutuable reference i.e. doesn't allow multiple concurrent queries, we instead use web::Data<PgPool> and use a shared reference to that pool.
 
Curl:
![curl](https://github.com/DaveMcMahon/Login-API/assets/16767214/39717b99-9363-4e72-a366-4e880f760222)

Saved Row:
<img width="933" alt="row" src="https://github.com/DaveMcMahon/Login-API/assets/16767214/8c6717f8-9479-4e9c-a3e6-a0423cf68ac7">
 
 

#### As the project grows, this ReadMe will get updated to included relevant frameworks and tools used.
