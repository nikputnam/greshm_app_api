
# server
rustup run nightly cargo run

# client test
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
