
# server

to start:

```sh
rustup run nightly cargo run
```

# client test

```sh
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' \
        -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend

#generates an error becaue token is for user 'nik'
curl -X POST --data '{ "from": "alex", "to": "nik", "amount": 1200.00 }' \
        -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
```

# TODO:

* Attach some storage -- could just be in memory to start
* Add 'new account' endpoint that saves password (if account doesn't exist)
* Check passwords on auth request
* Log transactions & update balances
* Check validity of spend txs
