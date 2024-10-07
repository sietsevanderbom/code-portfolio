# Design

Date: 3rd of September, 2024

## Overview
The goal of this project is to implement a Zero-Knowledge Proof (ZKP) based authentication system using the Chaum-Pedersen protocol. The system consists of a client and server communicating via gRPC, where the client can register and authenticate using ZKP without revealing the secret password.

## Project Structure
The project is organized into several components:

- proto: Protobuf definitions.
- zkp_auth: Library for ZKP protocol generated from proto definitions.
- zkp_server: gRPC server implementation.
- zkp_client: gRPC client implementation.
- utils: Utility functions for handling cryptographic operations.
- tests: Unit tests for the utils.

## ZKP Protocol Implementation
### Registration Process
1. Client: The client generates a secret password x and computes y1 = g^x and y2 = h^x using public generators g and h.
2. Client: Sends y1 and y2 to the server.
3. Server: Stores y1 and y2 associated with the user.

### Login Process
1. Commitment Step:
    - client: generates a random nonce k and computes commitments r1 = g^k and r2 = h^k.
    - client: sends r1 and r2 to the server.
2. Challenge Step:
    - Server: Generates a random challenge c and sends it to the client.
3. Response Step:
    - Client: Computes the response s = k - c * x (mod q) and sends it to the server.
4. Verification Step:
    - Server: Verifies the response by checking if r1 = g^s * y1^c and r2 = h^s * y2^c.
    - Server: If the verification is successful, the authentication is successful.

## Protobuf Definitions
The communication between the client and server is defined using Protocol Buffers. The protobuf schema includes messages for registration, authentication challenge, and authentication answer, as well as the corresponding gRPC service methods.

## BigUint

## variable naming
The short variable names in this assignment are in line with the protobuf definition and considered well known in cryptography so we'll use those in our code.

## Performance and Optimizations
We'll use BigUint for handling large integers and ensure proper serialization and deserialization.

In the context of zero-knowledge proofs (ZKPs) and cryptographic protocols, using negative numbers can be problematic, so we'll work with BigUint, not BigInt. Cryptographic operations typically require non-negative integers, especially when working with modular arithmetic, as negative numbers can introduce complexities and potential vulnerabilities.

## Extensibility and Integration

### Extending the Protocol
The current structure of the application with distinct library, server and client components allows for easy extension to support alternative versions of the protocol. This modular design ensures that other cryptographic techniques or additional authentication factors could be easily integrated.

### Integration
Containerization of the client and server using Docker and provide a Docker Compose setup for easy local deployment. We provide 'just' recipees to deploy the client and server on separate AWS instances. For this we use terraform with cloud-init and deploy an AWS container registry for the docker images. Recipees to destroy the setup are also provided.

### Documentation
We include a detailed README file with instructions for building, running, and testing the project. Include comments in the code to explain the logic and flow of the ZKP protocol.

## Conclusion
This design provides a comprehensive approach to implementing a ZKP-based authentication system using the Chaum-Pedersen protocol. By following this design, we can ensure a secure, efficient, and extensible authentication system that leverages modern cryptographic techniques and best practices in software development.

For more details, refer to the Design.md and Assignment.md files.
