FROM node:18-slim AS builder
WORKDIR /app
COPY package.json /app
COPY package-lock.json /app
RUN npm ci --only=production
COPY app.js /app/

FROM gcr.io/distroless/nodejs:18
WORKDIR /app
COPY --from=builder /app /app

CMD ["/app/app.js"]