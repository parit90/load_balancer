# **Pingora Proxy Server with Dynamic Load Balancing**

This project is a Rust-based proxy server leveraging the **Pingora** framework. It provides a mechanism for dynamic upstream management, load balancing, and routing for APIs. It features:

- **Dynamic addition and removal of upstream servers.**
- **Load balancing** using **RoundRobin** selection strategy.
- Graceful error handling when no upstreams are configured.
- **Modular project structure** for maintainability and scalability.

---

## **Features**

1. **Dynamic API Management**  
   - Add or remove upstream servers dynamically via REST APIs.
   - APIs for `apiA` and `apiB` endpoints.

2. **Load Balancing**  
   - Uses Pingora's load balancing with the **RoundRobin** algorithm.
   - Ensures fair distribution of requests across available upstream servers.

3. **Graceful Error Handling**  
   - Returns `502 Bad Gateway` with descriptive JSON error when no upstreams are available.

4. **Modular Structure**  
   - Code is organized into modules for better maintainability.

---

## **Project Structure**
```
. ├── src 
  │ ├── main.rs # Entry point of the application 
  │ ├── router # Router logic 
  │ │ ├── api.rs # Functions for managing upstreams 
  │ │ ├── router.rs # Router implementation 
  │ ├── handlers # Dynamic API handlers 
  │ │ ├── dynamic_api.rs # REST API for managing upstreams 
  │ ├── utils # Utility functions 
  │ ├── logging.rs # Logging utilities for upstream counts ├── Cargo.toml # Rust dependencies and configuration ├── README.md # Project documentation
```



---

## **Requirements**

- **Rust** (stable version)
- **Node.js server** for testing upstreams (optional for testing `/apiA` and `/apiB` endpoints)

---

## **Usage**

### **Running the Server**

1. **Clone the repository** and navigate to the project directory.
   ```bash
   git clone <repo-url>
   cd load_balancer

```cargo run


Add an upstream 
```curl -X POST "http://127.0.0.1:8080/add_upstream/<service_name>/<server_address>"
e.g.
```curl -X POST "http://127.0.0.1:8080/add_upstream/apiA/127.0.0.1:3001"


Remove an Upstream
```curl -X POST "http://127.0.0.1:8080/remove_upstream/<service_name>/<server_address>"


Test the Proxy
```curl -X GET "http://127.0.0.1:6188/apiA"