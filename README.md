# Simple HTTP Server in Rust

This project is a basic HTTP server implemented in Rust, capable of handling multiple connections using a thread pool. It serves static HTML files and simulates a delay for specific requests.

## Features

- **Multi-threaded**: Utilizes a thread pool to handle incoming connections concurrently.
- **Basic Routing**: Responds to requests for the root path (`/`) and a simulated sleep endpoint (`/sleep`).
- **Custom Error Handling**: Returns a 404 page for unsupported routes.

## Getting Started

### Prerequisites

- Rust (latest stable version) installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Running the Server

1. Clone the repository:

   ```bash
   git clone https://github.com/williammuchui/http_server.git 
   cd http_server
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the server:

   ```bash
   cargo run
   ```

   The server will start and listen on `127.0.0.1:7878`.

### Accessing the Server

Open your web browser or use a tool like `curl` to access the following endpoints:

- **Root Endpoint**: [http://127.0.0.1:7878/](http://127.0.0.1:7878/) - Returns `hello.html`.
- **Sleep Endpoint**: [http://127.0.0.1:7878/sleep](http://127.0.0.1:7878/sleep) - Waits for 5 seconds before returning `hello.html`.
- **Invalid Endpoint**: Any other path will return a 404 error page.

### File Structure

Ensure you have the following HTML files in your project directory:

- `hello.html`: Content served for the root and sleep endpoints.
- `404.html`: Content displayed for unsupported routes.

## Code Overview

The server is implemented in `main.rs`. Key components include:

- **Thread Pool**: Manages a pool of threads to handle requests concurrently.
- **Request Handling**: Parses incoming requests and routes them accordingly.
- **Response Generation**: Constructs and sends HTTP responses based on the request.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

