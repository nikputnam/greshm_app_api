#!/usr/bin/env bash

cat << 'EOM'
]$ token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
]$ echo $token
EOM
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
echo $token

cat << 'EOM'
]$ token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/signup`
]$ echo $token
EOM
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/signup`
echo $token

cat << 'EOM'
]$ token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
]$ echo $token
EOM
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
echo $token

cat << 'EOM'
]$ curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
EOM
curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend


cat << 'EOM'
]$ curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
EOM
curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend


cat << 'EOM'
]$ curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
EOM
curl  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance








exit 0;

592  echo $token
  593  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  594  history 
  595  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  596  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  597  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  598  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  599  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  600  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  601  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  602  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  603  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  604  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/nik/1
  605  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
  606  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
  607  token=`curl -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"123pws","rememberMe":false}' http://localhost:8000/auth`
  608  echo $token
  609  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
  610  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
  611  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
  612  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  613  pwd
  614  token=`curl -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"alex"}' http://localhost:8000/auth`
  615  token=`curl -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"alex","password":"123pws"}' http://localhost:8000/auth`
  616  echo $token
  617  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  618  token=`curl -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"alex","password":"123pws"}' http://localhost:8000/auth`
  619  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"alex","password":"123pws"}' http://localhost:8000/auth`
  620  echo $token
  621  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  622  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
  623  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  624  echo $token
  625  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  626  curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  627  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  628  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  629  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  630  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend/
  631  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  632  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  633  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  634  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  635  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  636  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  637  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  638  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  639  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  640  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  641  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  642  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  643  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  644  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  645  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  646  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  647  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  648  curl -X GET --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  649  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  650  curl -X GET --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  651  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  652  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  653  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  654  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  655  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  656  curl -X GET --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  657  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  658  history 
  659  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  660  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  661  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  662  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"bob","password":"cows"}' http://localhost:8000/auth`
  663  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  664  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  665  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  666  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"bob","password":"cows"}' http://localhost:8000/auth`
  667  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  668  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  669  curl -X POST --data '{ "from": "nik", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  670  curl -X POST --data '{ "from": "nik", "tbob "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  671  curl -X POST --data '{ "from": "nik", "tbob "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  672  history 
  673  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  674  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  675  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  676  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  677  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  678  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  679  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  680  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  681  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  682  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  683  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  684  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  685  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  686  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  687  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  688  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  689  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  690  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  691  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  692  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  693  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  694  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  695  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  696  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  697  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  698  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  699  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  700  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  701  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  702  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  703  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  704  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  705  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  706  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  707  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  708  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  709  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  710  pwd
  711  pwd
  712  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"alex","password":"cows"}' http://localhost:8000/auth`
  713  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  714  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  715  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  716  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  717  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  718  curl -X POST --data '{ "from": "bob", "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  719  curl -X POST --data '{ "from": "bob", "tnik "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  720  hisotry 
  721  history
  722  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  723  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  724  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  725  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  726  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  727  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  728  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  729  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  730  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  731  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  732  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  733  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  734  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  735  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  736  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  737  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  738  curl -X GET  -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  739  pwd
  740  pwd
  741* 
  742  pwd
  743  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup`
  744  echo $token
  745  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  746  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup`
  747  echo $token
  748  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  749  echo $token
  750  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup`
  751  echo $token
  752  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/login`
  753  echo $token
  754  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  755  echo $token
  756  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth` ; echo $token
  757  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup` ; echo $token
  758  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth` ; echo $token
  759  pwd
  760  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  761  curl -X POST --data '{ "from": "nik",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  762  curl -X POST --dGET'pt: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
  763  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup` ; echo $token
  764  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup` ; echo $token
  765  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup` ; echo $token
  766  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup` ; echo $token
  767  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/signup` ; echo $token
  768  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"alex","password":"cows"}' http://localhost:8000/signup` ; echo $token
  769  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows1"}' http://localhost:8000/auth`
  770  echo $token
  771  token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
  772  echo $token
  773  curl -X POST --data '{ "from": "alex",  "to": "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  774  curl -X POST --data '{ "from": "alex",  nik: "alex", "amount": 1200.00 }' -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend
  775  echo $token
  776  pwd
  777  history 
  778  history > test_script.sh
