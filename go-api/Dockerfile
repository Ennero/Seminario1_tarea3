# Etapa de construcción
FROM golang:1.22-alpine AS builder
WORKDIR /app
COPY main.go .
RUN go build -o api-go main.go

# Etapa final
FROM alpine:latest
WORKDIR /root/
COPY --from=builder /app/api-go .
EXPOSE 8080
CMD ["./api-go"]