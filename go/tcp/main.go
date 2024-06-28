package main

import (
	"fmt"
	"net"
	"os"
	"path"
	"path/filepath"
	"strings"
)

const distPath = "../../dist" 

func main() {
	// Listen on TCP socket for incoming connections
	listener, err := net.ListenTCP("tcp", &net.TCPAddr{Port: 3000})
	if err != nil {
		fmt.Println("Error listening:", err)
		return
	}
	defer listener.Close()
	fmt.Println("Server listening on port 3000")

	for {
		conn, err := listener.AcceptTCP()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}

		go handleConnection(conn)
	}
}

func handleConnection(conn *net.TCPConn) {
	defer conn.Close()

	requestBuf := make([]byte, 4096)
	n, err := conn.Read(requestBuf)
	if err != nil {
		fmt.Println("Error reading from connection:", err)
		return
	}

	requestLine := strings.SplitN(string(requestBuf[:n]), "\r\n", 2)[0]
	requestParts := strings.Fields(requestLine)
	if len(requestParts) < 3 {
		fmt.Println("Malformed request")
		return
	}

	method := requestParts[0]
	urlPath := requestParts[1]

	if method != "GET" {
		sendResponse(conn, "405 Method Not Allowed", "text/plain", []byte("Method Not Allowed"))
		return
	}

	// Resolve the file path based on the request URL
	filePath := path.Join(distPath, urlPath)

	// Check if the path is a directory
	fileInfo, err := os.Stat(filePath)
	if err == nil && fileInfo.IsDir() {
		// If it's a directory, serve the index.html file
		filePath = path.Join(filePath, "index.html")
	}

	// Check if file exists
	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		// Serve index.html for unmatched routes (client-side routing)
		filePath = path.Join(distPath, "index.html")
	}

	// Read file data
	data, err := os.ReadFile(filePath)
	if err != nil {
		sendResponse(conn, "500 Internal Server Error", "text/plain", []byte("Internal Server Error"))
		return
	}

	// Get content type based on file extension
	contentType := getContentType(filePath)
	sendResponse(conn, "200 OK", contentType, data)
}

func sendResponse(conn *net.TCPConn, status, contentType string, body []byte) {
	response := fmt.Sprintf("HTTP/1.1 %s\r\nContent-Length: %d\r\nContent-Type: %s\r\n\r\n", status, len(body), contentType)
	response = response + string(body)

	_, err := conn.Write([]byte(response))
	if err != nil {
		fmt.Println("Error writing to connection:", err)
		return
	}
}

func getContentType(filePath string) string {
	ext := strings.ToLower(filepath.Ext(filePath))
	switch ext {
	case ".html":
		return "text/html"
	case ".js":
		return "application/javascript"
	case ".css":
		return "text/css"
	case ".json":
		return "application/json"
	case ".png":
		return "image/png"
	case ".jpg", ".jpeg":
		return "image/jpeg"
	case ".gif":
		return "image/gif"
	case ".svg":
		return "image/svg+xml"
	case ".wav":
		return "audio/wav"
	case ".mp4":
		return "video/mp4"
	case ".woff":
		return "application/font-woff"
	case ".ttf":
		return "application/font-ttf"
	case ".eot":
		return "application/vnd.ms-fontobject"
	case ".otf":
		return "application/font-otf"
	case ".wasm":
		return "application/wasm"
	default:
		return "application/octet-stream"
	}
}
