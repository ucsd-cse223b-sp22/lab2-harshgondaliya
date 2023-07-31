# Distributed Key-Value Storage System (Lab 2)

<p align="center">
  <img src="https://github.com/ucsd-cse223b-sp22/lab2-harshgondaliya/blob/master/distributed-key-value-store.PNG" style="height: auto; width: 800px">
</p>

**Note:** `lab2-harshgondaliya/lab/src/lab2` contains the key code files for the project. The following documentation explains the major software artifacts in the project.

## BinStorage
A BinStore instance provides a `BinStoreClient` that allows the user's front-end to perform Storage API calls to the user's virtual key-value store. Moreover, the `BinStoreClient` also contains all the backend addresses, enabling the user's front-end to perform Storage API calls to any other user's virtual key-value store. A `Keeper` process runs in the background that iteratively performs `clock()` Storage API call to each backend server every one second. It ensures that logical clock at any backend server in a given iteration is set to at least the maximum logical clock value seen in last iteration plus 1.

## Tribbler
Tribbler maintains a special bin named `SIGNUP` in the bin storage that stores list of all the signed up users. Moreover, in each user's virtual key-value store, the following two keys are maintained to support various front-end operations:
1. `tribs`: stores tribs posted by user.
2. `following_log`: stores the history of `follow()` and `unfollow()` operations that a user has performed in the order in which they reached the backend.

**Note:** To keep the documentation short, minute details like `valid_username_check`, `valid_trib_len`, `is_signed_up`, `is_who_equal_whom` etc are skipped and only the key functionality is described below.

### sign_up()
The entire list of signed-up users is brought from the special `SIGNUP` bin. If the user is not found in signed-up users list then his name is appended to signed-up users list in the `SIGNUP` bin and the result is retured to client.

### list_users()
Each front-end caches a list of 20 users the first time it sees that at least 20 users have signed up. The front-end directly returns users cached in its memory if it contains 20 users. Otherwise, it will make RPC call to the user's bin and get the list of users.

### post()
A trib with its clock field set to the logical clock value is generated and appended to the user's `tribs` list in the user's virtual key value store.

### tribs()
The entire `tribs` list is fetched from the user's virtual key value store. The list is sorted in tribble order. If the user has posted less than equal to `MAX_TRIB_FETCH` tribs then all the sorted tribs are returned to client. Otherwise, all but the last `MAX_TRIB_FETCH` tribs in sorted list are removed from the backend, and the remaining `MAX_TRIB_FETCH` tribs are returned to the client tribble order.

**Note:** The removal of all but the last `MAX_TRIB_FETCH` tribs is just a garbage collection operation. Error message is not sent to the client if this garbage collection operation fails.

### follow()
A log is maintained in the user's virtual key value store that stores the entire history of `follow()` and `unfollow()` operations that a user has done.

The following sequence of operations are performed:

1. A log record for the current `follow()` operation is generated with the current logical clock value.
2. The log record is appended to the user's log list in the backend.
3. The user's log list is fetched from the backend.
4. All the `follow` and `unfollow` operations are traced right from the beginning to determine what result should be returned to the client for the current `follow()` operation that the client has invoked. Since all the log records are for the same user, our desired `follow()` operation can be uniquely identified using the logical clock value that is present in the log records.
5. After tracing the log, if it is found that the current `follow()` will exceed the `MAX_FOLLOWING limit`, then an error is returned to client. Otherwise, the result of current `follow()` determined after tracing history of logs is returned to the client.

### unfollow()
A log is maintained in the user's virtual key value store that stores the entire history of `follow()` and `unfollow()` operations that a user has done.

The following sequence of operations are performed:

1. A log record for the current `unfollow()` operation is generated with the current logical clock value.
2. The log record is appended to the user's log list in the backend.
3. The user's log list is fetched from the backend.
4. All the `follow` and `unfollow` operations are traced right from the beginning to determine what result should be returned to the client for the current `unfollow()` operation that the client has invoked. Since all the log records are for the same user, our desired `unfollow()` operation can be uniquely identified using the logical clock value that is present in the log records.
5. The result of current `unfollow()` determined after tracing history of logs is returned to the client.

### is_following()
A log is maintained in the user's virtual key value store that stores the entire history of `follow()` and `unfollow()` operations that a user has done.

The following sequence of operations are performed:

1. The user's log list is fetched from the backend.
2. All the follow and unfollow operations are traced right from the beginning to end to determine whether `who` is following `whom` at the end of all the operations present in the log list.
3. The result after tracing history of logs is returned to the client.

### following()
A log is maintained in the user's virtual key value store that stores the entire history of `follow()` and `unfollow()` operations that a user has done.

The following sequence of operations are performed:
1. The user's log list is fetched from the backend.
2. All the `follow` and `unfollow` operations are traced right from the beginning to end to determine all the users that the provided user is following after completion of all the operations present in the log list.
3. The result after tracing history of logs is returned to the client.

### home()
The following operations are performed:
1. The entire `tribs` list of user is fetched from backend. The list is sorted in tribble order. If the user has posted less than equal to `MAX_TRIB_FETCH` tribs then all the sorted tribs are appended to a `home_timeline` list. Otherwise, all but the last `MAX_TRIB_FETCH` tribs in sorted list are removed from the backend, and the remaining `MAX_TRIB_FETCH` tribs are returned to the client tribble order.
2. The same three log operations mentioned above for `following()` function are performed to get the list of all users that the current user is following. Additionally, as a garbage collection step, all the unrequired log entries are removed from the backend. If this garbage collection step fails then error is not retured to client.

**Note:** Unrequired entries are the ones that whose presence does not affect the outcome derived by tracing history of follow and unfollow operations. Thus, while performing `follow()` or `unfollow()` if a user does not find his log record in the log then he can conclude that his operation is invalid and can return error to client. 

3. Step 1 mentioned above is performed for each user that the current user is following.
4. Finally, home_timeline list is sorted in tribble order. If it has less than equal to `MAX_TRIB_FETCH` tribs then all the sorted tribs are returned. Otherwise, the last `MAX_TRIB_FETCH` are returned in tribble order.

## Garbage Collection
Two kinds are garbage collection are being done:
1. **Removing the tribs older than MAX_TRIB_FETCH tribs**: This is done in `tribs()` and `home()` function calls as described above. To avoid aggressive garbage collection invocations, we made a design choice to do it only in the `home()` function call and not in every `post()` function call.
2. **Removing the unrequired log entries in following_log**: This is done in `home()` function call as described above. To avoid aggressive garbage collection invocations, we made a design choice to do it only in the `home()` function call and not in every `follow()`, `unfollow()`, `is_following()`, or `following()` function call.
