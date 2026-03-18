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
		
		// El JSON con la estructura solicitada
		json.NewEncoder(w).Encode(map[string]string{
			"api": "GoLang",
			"mensaje": "Respuesta exitosa desde la API en Go",
			"estado": "Activo",
		})
	})

	log.Println("API en Go escuchando en el puerto 8080...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}