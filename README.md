A basic API backend, made during week 4 of Utah Tech University's Codeschool.

#### `GET /movies/<username>`
Get a list of watched movies created by `<username>`.
On success, returns status 200 and a JSON.
```json
[
    {
        "movie_id": "<HASHED KEY>"
        "title": "Gone with the Wind",
        "rating": 6.2
    },
    {
        "movie_id": "<HASHED KEY>"
        "title": "Casablanca",
        "rating": 8.0
    }
]
```
The "rating" field is optional and may not exist.

#### `GET /movies/<username>/<movie_id>`
Get the movie record by `<username>` with the id `<movie_id>`.
On success, returns status 200 and a JSON.
```json
{
    "movie_id": "<HASHED KEY>",
    "title": "Gone with the Wind",
    "rating": 6.2
}
```
The "rating" field is optional and may not exist.

#### `POST /movies/<username>?title=<title>&rating=<rating>`
Create a movie record for `<username>`.
`rating` is an optional parameter.
On success, returns status 200 and a JSON.
```json
{
    "movie_id": "<HASHED KEY>",
    "title": "<title>",
    "rating": "<rating>"
}
```
The "rating" field is optional and may not exist.

#### `PUT /movies/<username>/<movie_id>?title=<title>&rating=<rating>`
Update a movie record for `<username>`.
`rating` is an optional parameter.
On success, returns status 200.

#### `DELETE /movies/<username>/<movie_id>`
Delete a movie record for `<username>`.
On success, returns status 200.
