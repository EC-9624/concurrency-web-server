package main

import (
	"fmt"
	"net"
	"strconv"
)

func main() {
	// Port to listen on
	port := "8080"

	// Message to send in response
	message := "Hello, World!"

	// Parse the port number
	portNum, err := strconv.Atoi(port)
	if err != nil {
		fmt.Println("Error parsing port:", err)
		return
	}

	// Listen on TCP socket for incoming connections
	listener, err := net.ListenTCP("tcp", &net.TCPAddr{Port: portNum})
	if err != nil {
		fmt.Println("Error listening:", err)
		return
	}
	defer listener.Close()
	fmt.Println("Server listening on port", port)

	// Loop to accept connections
	for {
		conn, err := listener.AcceptTCP()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}

		// Handle connection in a separate goroutine
		go handleConnection(conn, message)
	}
}

func handleConnection(conn *net.TCPConn, message string) {
	defer conn.Close()

	// Simulate processing time (optional)
	// time.Sleep(time.Second)

	// Build the HTTP response (manually)
	response := fmt.Sprintf("HTTP/1.1 200 OK\r\nContent-Length: %d\r\n\r\n%s", len(message), message)

	// Write the response to the connection
	_, err := conn.Write([]byte(response))
	if err != nil {
		fmt.Println("Error writing to connection:", err)
		return
	}

}
