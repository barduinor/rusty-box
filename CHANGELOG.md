### 0.4.0-alpha
* Auth now supports:
    * Client Credentials Grant (CCG)
    * Developer Token
    * Oauth 2.0

### 0.3.0-alpha
### Refactor:
Completed refactoring of the error handle.

Now Auth implements it's own error handle and Client implements it's own error handle.

Created 2 different rqwest clients implementations. One for Auth and one for Client.

Added re-exports at lib level so use statements are cleaner.


* Auth
    * AuthClient
    * AuthError
    * AuthErrorResponse
* Client
    * Client
    * ClientError
    * ClientErrorResponse




## 0.2.0-alpha
### Endpoints:
- Added `users_api::create`
- Added `users_api::delete`
- Added `users_api::id`
- Added `users_api::list`
- Added `users_api::me`
- Added `users_api::terminate_sessions_by_groups_ids`
- Added `users_api::terminate_sessions_by_users_ids`
- Added `users_api::terminate_sessions_by_users_logins`
- Added `users_api::update`
### Notes:
- Removed way to many structures to handle passing parameters to the users api.
- Added samples for each endpoint.
- Cleaned up unused code.


## 0.1.0-alpha

- rusty-box first release

### Auth
- Developer Token
- Client Credentials Grant (CCG)

### User API
- users::me
- users::list