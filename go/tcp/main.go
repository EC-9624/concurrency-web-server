package main

import (
	"fmt"
	"net"
)

func main() {

	message := "Hello, World!"

	// Listen on TCP socket for incoming connections
	listener, err := net.ListenTCP("tcp", &net.TCPAddr{Port: 3000})
	if err != nil {
		fmt.Println("Error listening:", err)
		return
	}
	defer listener.Close()
	fmt.Println("Server listening on port", 3000)

	for {
		conn, err := listener.AcceptTCP()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}

		go handleConnection(conn, message)
	}
}

func handleConnection(conn *net.TCPConn, message string) {
	defer conn.Close()

	
	// Build the HTTP response 
	response := fmt.Sprintf("HTTP/1.1 200 OK\r\nContent-Length: %d\r\n\r\n%s", len(message), message)

	// Write the response to the connection
	_, err := conn.Write([]byte(response))
	if err != nil {
		fmt.Println("Error writing to connection:", err)
		return
	}

}
