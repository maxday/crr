FROM golang as builder
WORKDIR /app
COPY main.go go.mod /app/
RUN CGO_ENABLED=0 go build -ldflags '-w -s' -o cloud-run-go

FROM gcr.io/distroless/static-debian11
COPY --from=builder /app/cloud-run-go /cloud-run-go

CMD ["/cloud-run-go"]