package main

import (
	"fmt"
	"net/http"
	"os"
	"path"
	"path/filepath"
	"strings"
)
const distPath = "../../dist/" 

func main() {
	port := "3000"

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
    
		// Resolve the file path based on the request URL
		filePath := path.Join(distPath, r.URL.Path)

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
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		// Get content type based on file extension
		contentType := getContentType(filePath)
		w.Header().Set("Content-Type", contentType)

		// Write response
		w.Write(data)
	})

	fmt.Printf("Server listening on port %s\n", port)
	http.ListenAndServe(fmt.Sprintf(":%s", port), nil)
}

// getContentType determines the MIME type of the file based on its extension
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
