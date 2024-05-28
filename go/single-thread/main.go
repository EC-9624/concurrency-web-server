package main

import (
	"fmt"
	"net/http"
)

func helloHandler(w http.ResponseWriter, r *http.Request) {
  fmt.Fprintf(w, "Hello, world!")
}

func main() {
  http.HandleFunc("/", helloHandler)
  fmt.Println("Server listening on port 3000")
  http.ListenAndServe(":3000", nil)
}
