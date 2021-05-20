# RustGraphQL 
```diff
! WiP for personal website in rust
-
+
```
# Creating GraphQL backend using Rust
Each componenet  of the architecture answers several questions that may arise when implementing GraphQL API. 

## Service
 * Auth-Service   (RUST)
 * Post-Service   (RUST)
 * Apollo-Server (JS)

GraphQL backends used: **Juniper**
  * Supports apollo federation
  * PostgreSQL for persistence layer imlementation
  * JWT for auth

#Tech Stack
main technolgoies used dor the project
 * Language                 - Rust
 * GraphQL library          - Async-graphql
 * Single GraphQL Endpoint  - Apollo Server
 * Web Framework            - actix-web
 * Database                 - PostgreSQL
 * Container Orchestration  - Docker

Rust Libraries
 * Diesel             - ORM
 * argonautica        - Password Hashing Library
 * jsonwebtoken       - JWT library
 * Testcontainers-rs  - Testing Library

# Common tasks when creating GrahpQL API

# Microservices to a single endpoint using Apollo Server and Apollo Federation. 
