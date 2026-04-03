package main
 
import (
	"fmt"
	"log"
	"net/http"
)
 
func main() {
	http.HandleFunc("/", homeHandler)
	http.HandleFunc("/health", healthHandler)
 
	port := "8080"
	log.Printf("Server starting on port %s...", port)
	if err := http.ListenAndServe(":"+port, nil); err != nil {
		log.Fatal(err)
	}
}
 
func homeHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello from Go! 🚀")
}
 
func healthHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintln(w, `{"status": "ok"}`)
}
