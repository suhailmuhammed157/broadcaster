# 📡 Broadcaster — Ultra-Fast Real-Time Messaging Server

Broadcaster is a high-performance backend server built with Rust and the Actix Web framework, engineered for real-time message broadcasting over WebSockets. Designed for speed, scalability, and reliability, it allows registered platforms to instantly push updates, notifications, or data to thousands of connected clients with minimal latency.

## 🚀 Key Highlights:

Blazing Fast: Built with Rust and Actix for top-tier speed and concurrency.

WebSocket Powered: Seamless, persistent, low-latency connections for real-time communication.

Platform Registration: Secure platform-based broadcasting to specific user groups or services.

Scalable Architecture: Designed to support high-throughput message delivery in large-scale systems.

Reliable Delivery: Ensures message integrity and consistent communication between servers and clients.

## 🧠 Use Cases:

- Live sports score updates
- Stock market tickers
- Multiplayer game servers
- Collaborative applications (e.g., whiteboards, chat)
- IoT device communication

Built for performance-driven systems, Broadcaster is ideal for any real-time application demanding high speed and reliability. Whether it's powering a live dashboard or synchronizing thousands of clients, Broadcaster delivers—fast.

## 🚀 Features

- ✅ Platform registration with JWT token issuance
- ✅ Message broadcasting for authenticated platforms
- ✅ Real-time client communication via WebSocket
- ✅ Simple `.env`-based configuration

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Cargo](https://doc.rust-lang.org/cargo/)
- `.env` file for environment configuration

---

## 🛠️ Installation

### 1. **Clone the repository**

```bash
git clone https://github.com/suhailmuhammed157/broadcaster.git
cd broadcaster
```

### 2. **Create a `.env` file in the project root with the following variables:**

```bash
HOST=127.0.0.1
PORT=8080
SECRET=your_jwt_secret_key

```

### 3. **Run the application**

```bash
cargo run

```

Server will start at: `http://127.0.0.1:8080`

## 🔐 API Endpoints

📥 `POST /platform/register` : Registers a platform and returns a JWT token.

Request body:

```bash
{
  "platform_name": "demo-platform"
}

```

Response:

```bash
{
    "Data": {
        "platform_id": 1,
        "token": "your.jwt.token.here"
    },
    "Message": "Platform added with id 1",
    "Status": 200
}

```

📤 `POST /platform/broadcast` : Broadcasts a json type message to all connected WebSocket clients under the given platform.

Headers : `Authorization: Bearer <your-jwt-token>`

Request body:

```bash
{
  "message": { "subject": "Hello, world!"}
}

```

Response:

```bash
{
    "Message": "Message broadcasted successfully",
    "Status": 200
}

```

🔌 `GET /ws?platform=platform_name` : WebSocket endpoint that clients connect to in order to receive messages.

Example : `ws://127.0.0.1:8080/ws?platform=demo-platform`

Clients must supply the platform query parameter to receive messages specific to that platform.

### 📌 Notes

- Make sure .env is present before running cargo run.

- JWTs are time-limited (e.g. 1-minute expiration by default).

- Platform name is used to match WebSocket channels.

- Each connected client will receive all messages broadcast to its platform.

## 🐳 Run with Docker (Using Makefile)

You can easily build, run, and manage the Broadcaster Docker container using the provided Makefile.

### 1. Prepare your .env file

Make sure you have a .env file in the project root with these variables:

```bash
HOST=0.0.0.0
PORT=9090
SECRET=your_jwt_secret_key
```

Note: Setting `HOST=0.0.0.0` allows the server to listen on all interfaces inside the container.

### 2. Build the Docker image

```bash
make build
```

### 3. Run the Docker container

```bash
make run
```

The server will be accessible at: `http://localhost:9090`

### 4. View logs

```bash
make logs
```

### 5. Stop and remove the container

```bash
make stop
```

### 6. Rebuild and restart container

```bash
make restart
```

### 7. Shell into the running container (for debugging)

```bash
make shell
```
