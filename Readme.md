# database cluster

There should a file called 'docker.cluster' with the URI for the foundation db cluster.  
It's contents should look like e.g. 'docker:docker@127.0.0.1:4550'

You can use the scripts in `foundationdb/packaging/docker/samples/local` in the `https://github.com/apple/foundationdb` repo to start and stop foundationdb in a docker image.

# server

to start local server for dev and testing:

```sh
rustup run nightly cargo run
```

to start production server:

```sh
ROCKET_ENV=production rustup run nightly cargo run
```


# client test

run ./test_script.sh to see a more complete sequence of example API calls including error cases

```sh
token=`curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"username":"nik","password":"cows"}' http://localhost:8000/auth`
curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/recent
curl -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/balance
curl -X GET -H 'Accept: application/json' -H "Authorization: ${token}" http://localhost:8000/spend/bob/1200.00
```

# remote server

to install on server:

1. Set up Ansible on local machine

```
sudo apt update
sudo apt install software-properties-common
sudo apt apt-add-repository ppa:ansible/ansible
sudo apt update
sudo apt install ansible
```

2. Create Remote Server
 - Ubuntu 18.04
 - SSH certificate root access

3. Add server to Ansible inventory file (/etc/ansible/hosts)

```
[greshm_api]
server1 ansible_host=<remote_server_hostname>

[greshm_api:vars]
ansible_python_interpreter=/usr/bin/python3
```

4. Run ansible playbook

```
ansible-playbook ansible_setup_remote_server.yml
```

5. SSH into remote server

```
ssh greshm@<remote_server_hostname>
```

6. Install Rust

```
$ curl https://sh.rustup.rs -sSf | sh

7. Clone the git repo.

$ git clone https://github.com/nikputnam/greshm_app_api.git
$ cd greshm_app_api
$ rustup install nightly
```

8. Run production server
```
$ ROCKET_ENV=production rustup run nightly cargo run
```


# TODO:

* Write tests
* Attach persistent storage 
