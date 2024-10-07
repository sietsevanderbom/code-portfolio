# zkp-auth and PoC

This project is a proof of concept for Zero Knowledge Proof (ZKP) based authentication system. The project consists of a library for ZKP protocol, a gRPC zkp_server, and a gRPC zkp_client.

## Project Documentation

The assignment underlying this project is in the `docs/Assignment.md` file. The design of the project is in the `docs/Design.md` file.

## Quick start

Following basics need to be installed on your system:
- rust, install with [rustup](https://www.rust-lang.org/tools/install)
- [docker](https://docs.docker.com/get-started/get-docker/)
- [terraform](https://developer.hashicorp.com/terraform/tutorials/aws-get-started/install-cli)

For easily running project locally and on AWS:
Install the [just](https://just.systems/) command runner and [awscli](https://aws.amazon.com/cli/).
```sh
// linux
apt install just
apt install awscli -y

// mac
brew install just
brew install awscli
```
Other system installation for 'just' [here](https://github.com/casey/just?tab=readme-ov-file#packages).

## Recipes
To see what recipes are available, run `just` in the root of the project. This will show the following:
```sh
Available recipes:
    test                 # Run tests
    cargo-clean-build    # Build/ rebuild entire project (regenerates the proto file)
    cargo-release        # Build a release
    cargo-server         # Run local zkp_server binary
    cargo-client         # Run local zkp_client binary
    local-up             # Spin up local zkp_server and zkp_client containers
    local-down           # Reverse 'local-up', takes down the containers
    aws-up               # Provision zkp_server and zkp_client on AWS
    aws-down             # Reverse 'aws-up', takes down all, i.e. AWS machines and registry
    ssh machine='server' # Usable after 'aws-up' completed. Ssh into server: 'just ssh' or client: 'just ssh client'
```

Each recipe can be run with `just <recipe-name>`. For example, to run the `local-up` recipe, run `just local-up`.

## Running on AWS

### AWS Access Keys
For `just aws-up` to work you need AWS Access Keys with sufficient permissions for a region to setup EC2 machines and an ECR. Set these credentials in the `.env` file in the root of the project:
```sh
AWS_ACCESS_KEY_ID="<your-access-key>"
AWS_SECRET_ACCESS_KEY="<your-secret-access-key>"
AWS_REGION="<your-region>"
```

It's been successfully deployed on 'eu-central-1' region, but with sufficient permissions it should work in any region.

### Details on functionally testing the AWS deployment
The `just aws-up` recipe logs will end with:
```sh
Apply complete! Resources: 9 added, 0 changed, 0 destroyed.

Outputs:

zkp_client_public_ip = "3.124.117.119"
zkp_server_public_ip = "18.197.229.156"
```

After these logs both machines are still being provisioned in line with their cloud-init configs, which takes a couple of minutes. You can already ssh into the machines.

Ssh into the EC2 server with `just ssh`, and then:
```sh
ubuntu@ip-10-0-1-88:~$ docker ps
CONTAINER ID   IMAGE                                                                 COMMAND                  CREATED         STATUS         PORTS                                           NAMES
f344be58ae59   947086469081.dkr.ecr.eu-central-1.amazonaws.com/zkp-repo:zkp_server   "/usr/local/bin/zkp_…"   7 minutes ago   Up 7 minutes   0.0.0.0:50051->50051/tcp, :::50051->50051/tcp   zkp_server

// If no containers are shown to be running yet, then you might inspect the
// cloud-init progress with: `sudo vim /var/log/cloud-init-output.log`

ubuntu@ip-10-0-1-88:~$ docker logs zkp_server
Server is up & running

// means the server is ready to accept connections
```

Ssh into the EC2 client with `just ssh client`, then:
```sh
ubuntu@ip-10-0-1-77:~$ docker ps
CONTAINER ID   IMAGE                                                                 COMMAND                  CREATED         STATUS         PORTS                                           NAMES
1feea0a5407c   947086469081.dkr.ecr.eu-central-1.amazonaws.com/zkp-repo:zkp_client   "/usr/local/bin/zkp_…"   8 minutes ago   Up 8 minutes   0.0.0.0:50051->50051/tcp, :::50051->50051/tcp   zkp_client

// If no containers are shown to be running yet, then you might inspect the
// cloud-init progress with: `sudo vim /var/log/cloud-init-output.log`

ubuntu@ip-10-0-1-77:~$ docker logs zkp_client
Press any key to start the registration process, or 'q' to quit
ubuntu@ip-10-0-1-77:~$ docker attach zkp_client

// This is just an empty line, but the client is waiting for any key, or 'q' to quit.
// So after pressing e.g. <space>, it continues with the registration process:
Enter your user-id:
345
Enter your user-secret (a very large number):
999999999998888888888888888888888888888888888
You successfully logged in with session id: "7LPmRNww3"
ubuntu@ip-10-0-1-77:~$
```

### AWS Cleanup
To clean up the AWS resources, run `just aws-down`. Successful destroy logs will contain:
```sh
// At the top: the registry is destroyed
Destroy complete! Resources: 1 destroyed.
// At the bottom: the EC2 machines, key-pair, and security group are destroyed
Destroy complete! Resources: 9 destroyed.
```