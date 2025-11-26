#!/bin/bash

# Hot reloading development script for Momenta SSR + WASM

echo "🔥 Starting Momenta SSR + WASM Hot Reloading Development Server"
echo "=================================================="

# Function to build WASM
build_wasm() {
    echo "🔧 Building WASM..."
    wasm-pack build --target web --dev
    if [ $? -eq 0 ]; then
        echo "✅ WASM build successful"
    else
        echo "❌ WASM build failed"
    fi
}

# Function to notify server of changes via HTTP
notify_reload() {
    curl -s -X POST http://localhost:3000/api/reload > /dev/null 2>&1 || true
}

# Function to start server
start_server() {
    echo "🚀 Starting server..."
    cargo run --features server &
    SERVER_PID=$!
    echo "Server PID: $SERVER_PID"
}

# Function to stop server
stop_server() {
    if [ ! -z "$SERVER_PID" ]; then
        echo "🛑 Stopping server (PID: $SERVER_PID)..."
        kill $SERVER_PID 2>/dev/null
        wait $SERVER_PID 2>/dev/null
    fi
}

# Function to restart server
restart_server() {
    stop_server
    sleep 1
    start_server
}

# Initial build and start
build_wasm
start_server

echo ""
echo "🎯 Development server running at http://localhost:3000"
echo "📝 Watching for changes in src/..."
echo "Press Ctrl+C to stop"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    stop_server
    exit 0
}

# Set up trap for cleanup
trap cleanup INT TERM

# Simple file watching loop
LAST_WASM_CHANGE=""
LAST_SERVER_CHANGE=""

while true; do
    # Check for WASM-related changes (lib.rs, components.rs, and components directory)
    CURRENT_WASM_CHANGE=$(find src/lib.rs src/components.rs src/components/ -type f -exec stat -f "%m" {} \; 2>/dev/null | sort -n | tail -1)
    if [ "$CURRENT_WASM_CHANGE" != "$LAST_WASM_CHANGE" ] && [ ! -z "$CURRENT_WASM_CHANGE" ]; then
        echo "📦 WASM files changed, rebuilding..."
        build_wasm
        notify_reload
        LAST_WASM_CHANGE="$CURRENT_WASM_CHANGE"
    fi
    
    # Check for server-related changes (main.rs, Cargo.toml)
    CURRENT_SERVER_CHANGE=$(find src/main.rs Cargo.toml -type f -exec stat -f "%m" {} \; 2>/dev/null | sort -n | tail -1)
    if [ "$CURRENT_SERVER_CHANGE" != "$LAST_SERVER_CHANGE" ] && [ ! -z "$CURRENT_SERVER_CHANGE" ]; then
        echo "🔄 Server files changed, rebuilding..."
        restart_server
        LAST_SERVER_CHANGE="$CURRENT_SERVER_CHANGE"
    fi
    
    sleep 1
done
