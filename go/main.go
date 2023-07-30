package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", helloWorld)
	http.ListenAndServe(":8080", nil)
}

func helloWorld(w http.ResponseWriter, r *http.Request) {
	fmt.Fprint(w, "Hello world from Go!")
}
