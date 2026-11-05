# APIs Go y Rust con cliente HTML y Docker Compose

Demostración de dos APIs que devuelven datos de identificación en /get-data. La primera usa net/http de Go y la segunda Rust/Axum. Un cliente HTML permite consultarlas.

## Requisitos

Docker y Compose v2 para las APIs. Python 3 puede servir el frontend estático; no es una dependencia del backend.

## Ejecución

Los comandos parten de la raíz del repositorio, salvo donde se indica otra terminal.

```bash
docker compose up --build -d

# Servir el cliente desde otra terminal:
python -m http.server 8000 --directory frontend
```

## Recorrido de demostración

Abrir http://localhost:8000. Probar directamente http://localhost:8081/get-data (Go) y http://localhost:8082/get-data (Rust). Consultar docker compose logs para el arranque y docker compose down para detener sin borrar volúmenes.

## Organización

Carpetas de la entrega: [`frontend/`](frontend/), [`go-api/`](go-api/), [`rust-api/`](rust-api/).

## Alcance y límites

El frontend no implementa un sistema de componentes complejo. Los backends de esta entrega devuelven datos de ejemplo y no requieren una base de datos.

## Contexto y licencia

Trabajo académico o de aprendizaje conservado en el portafolio de [Enner Mendizabal](https://github.com/Ennero). Los créditos de los trabajos en grupo se conservan en sus entregas y manuales.

Consultar [LICENSE](LICENSE) para la licencia de la raíz. Las dependencias y los componentes con avisos o licencias propios conservan sus condiciones.
