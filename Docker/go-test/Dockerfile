FROM golang:1.26.1-alpine3.23 as Builder

WORKDIR /app

COPY go.mod .

RUN go mod download

COPY . .

RUN go build -o api-golang

##
FROM scratch

COPY --from=Builder /app/api-golang api-golang

CMD ["./api-golang"]
