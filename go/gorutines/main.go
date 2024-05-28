package main

import (
	"fmt"
	"log"
	"net/http"
)


func handler(w http.ResponseWriter, r *http.Request) {
  fmt.Fprintf(w, "Hello, World!")
}

func main() {
  http.HandleFunc("/", handler)
  fmt.Println("Server listening on port 3000")
  log.Fatal(http.ListenAndServe(":3000", nil))
}
