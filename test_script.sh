#!/usr/bin/env bash

echo "Attempt to log in as nik.  (But account doesn't exist)"
cat << 'EOM'
]$ curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth
EOM
curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth
echo; echo;

echo "register account: 'nik'"
cat << 'EOM'
]$ curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/signup
EOM
curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/signup
echo; echo;


echo "register account: 'bob'"
cat << 'EOM'
]$ curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"bob","password":"123pws","rememberMe":false}' http://localhost:8000/signup
EOM
curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"bob","password":"123pws","rememberMe":false}' http://localhost:8000/signup
echo; echo;


echo "get JWT token for 'nik'"
cat << 'EOM'
]$ token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
]$ echo $token
EOM
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
echo $token
echo; echo;

echo "Try to spend as bob"
cat << 'EOM'
]$ curl -X POST --data '{ "from": "bob", "to": "nik", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
EOM
curl -X POST --data '{ "from": "bob", "to": "nik", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
echo
echo; echo;

echo "Try to spend as as nik, to alex"
cat << 'EOM'
]$ curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
EOM
curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
echo
echo; echo;


echo "check balance"
cat << 'EOM'
]$ curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
EOM
curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
echo
echo; echo;

echo "Try to spend as as nik, to bob"
cat << 'EOM'
]$ curl -X POST --data '{ "from": "nik", "to": "bob", "amount": 1.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
EOM
curl -X POST --data '{ "from": "nik", "to": "bob", "amount": 1.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
echo
echo; echo;

echo "Wait 1 s"
sleep 1
echo; echo;

echo "Try to spend as as nik, to bob, after receipt of basic income"
cat << 'EOM'
]$ curl -X POST --data '{ "from": "nik", "to": "bob", "amount": 1.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
EOM
curl -X POST --data '{ "from": "nik", "to": "bob", "amount": 1.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
echo
echo; echo;

echo "check balance for nik"
cat << 'EOM'
]$ curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
EOM
curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
echo
echo; echo;


echo "get JWT token for 'bob'"
cat << 'EOM'
]$ token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"bob","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
]$ echo $token
EOM
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"bob","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
echo; echo;

echo "check balance as bob"
cat << 'EOM'
]$ curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
EOM
curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
echo
echo; echo;


echo "recent as bob"
cat << 'EOM'
]$ curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
EOM
curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
echo
echo; echo;

