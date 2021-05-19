# RustGraphQL 

# Creating GraphQL backend using Rust
Each componenet  of the architecture answers several questions that may arise when implementing GraphQL API. 

## Service

 * Auth-Service   (RUST)
 * Crop-Service   (RUST)
 * To-Do Service  (RUST)
 * Apollo-Server (JS)

 Two Main GraphQL backends in Rust: *Juniper* and *Async-graphql*, only Async-graphql supports Apollo Federation
 . PostgreSQL is used for persistence layer implemntation, JWT - For auth, and Kafka for asynch messaging.


#Tech Stack
main technolgoies used dor the project
 * Language                 - Rust
 * GraphQL library          - Async-graphql
 * Single GraphQL Endpoint  - Apollo Server
 * Web Framework            - actix-web
 * Database                 - PostgreSQL
 * Event Streaming Platform - Apache Kafka
 * Container Orchestration  - Docker

Rust Libraries
 * Diesel             - ORM
 * rust-rdkafka       - Kafka Client
 * argonautica        - Password Hashing Library
 * jsonwebtoken       - JWT library
 * Testcontainers-rs  - Testing Library

# Common tasks when creating GrahpQL API

# Microservices to a single endpoint using Apollo Server and Apollo Federation. 
