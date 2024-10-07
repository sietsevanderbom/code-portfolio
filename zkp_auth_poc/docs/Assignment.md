# zkp_auth

## implemenentation ZKP Protocol
New authentication protocol, alternative to password hashing: Zero-Knowledge Proof (ZKP) in an authentication schema.

1. Implementing the ZKP Protocol, and
2. a Proof-of-Concept application that utilizes the protocol to register and authenticate users.

The ZKP protocol:
"Cryptography: An Introduction (3rd Edition) Nigel Smart" page 377 section "3. Sigma Protocols" subsection "3.2. Chaum–Pedersen Protocol."

Adapt the protocol to support 1-factor authentication: exact matching of a number (registration password) stored during **registration** and another number (login password) generated during the **login** process.

## Registration Process
The **prover (client)** has a secret password x (i.e. it is a number) and wishes to register it with the **verifier (server)**. To do that, they calculate y1 and y2 using public g and h and the secret x and sends to the verifier y1, y2.

## Login Process
The login process is done following the below ZKP Protocol between the Prover the authenticating party and the Verifier the server running the authentication check:

Note: y1 = g^x and y2 = h^x, i.e. g, h, y1 and y2 are all public information.

1. commitment step: prover generates random k and sends (r1, r2) = (g^k, h^k) to verifier.
2. challenge step: verifier generates random c and sends back a random challenge c.
3. response step: prover computes s, which is s = k - c * x (mod q) and sends s to verifier.
4. authentication success: output succesful authentication if r1 = g^s * y1^c and r2 = h^s * y2^c.

Main deliverable:
Design (README.md) and write the code that implements the ZKP Protocol outlined above. Solution should be implemented as server and client using gRPC protocol according to the provided interface described in “protobuf” schema. The code should implement very simple server and client applications.
Re design: We would like to see how you are thinking about and approach the problem, so a simple description of your approach and how you’d extend it or integrate would be helpful.

Bonus:
In a working implementation:

1. Unit tests, where appropriate.
2. Functional test of the ZKP Protocol.
3. A setup to run the Client and the Server.
4. Use Rust as language for the implementation.
5. Performance and optimizations.
6. Coverage of test cases (not code coverage).
7. Code soundness.
8. Code organization.
9. Code quality.
10. Well documented code.
11. Each instance runs in a separate docker container and has a docker compose to run the setup.
12. There is code to deploy the two containers in AWS. The client in one machine and the server in another machine
13. Implement two flavor: One with exponentiations (as described in the book) and one using Elliptic Curve cryptography (see for example this ZKP implementation in Rust).
14. Allow using “BigInt” numbers.

## protobuf definition

```protobuf
syntax = "proto3";
package zkp_auth;

message RegisterRequest {
    string user = 1;
    int64 y1 = 2;
    int64 y2 = 3;
}

message RegisterResponse {}

message AuthenticationChallengeRequest {
    string user = 1;
    int64 r1 = 2;
    int64 r2 = 3;
}

message AuthenticationChallengeResponse {
    string auth_id = 1;
    int64 c = 2;
}

message AuthenticationAnswerRequest {
    string auth_id = 1;
    int64 s = 2;
}

message AuthenticationAnswerResponse {
    string session_id = 1;
}

service Auth {
    rpc Register(RegisterRequest) returns (RegisterResponse) {}
    rpc CreateAuthenticationChallenge(AuthenticationChallengeRequest) returns
(AuthenticationChallengeResponse) {}
    rpc VerifyAuthentication(AuthenticationAnswerRequest) returns (AuthenticationAnswerResponse)
{}
}
```
