package main

import (
	"encoding/json"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/get-data", func(w http.ResponseWriter, r *http.Request) {
		// Habilitar CORS para evitar errores en el frontend
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Content-Type", "application/json")
		
		// El JSON exacto solicitado en la rúbrica para la API 1
		json.NewEncoder(w).Encode(map[string]string{
			"Instancia":  "Instancia #1 - API #1",
			"Curso":      "Seminario de Sistemas 1",
			"Estudiante": "Estudiante - 202302220",
			"Lenguaje":   "Go",
		})
	})

	log.Println("API en Go escuchando en el puerto 8080...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}