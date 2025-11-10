# Blob Service Docker Deployment

This directory contains Docker configurations for deploying the blob service with integrated nginx file serving.

## Services Included

1. `blob` - The blob service application
2. `blob-nginx` - Nginx server for secure file serving

## Directory Structure

```
docker/
├── blob.yml           # Docker compose file for blob service
├── nginx.conf         # Nginx configuration
├── private/
│   └── uploads/       # File storage directory (mapped to blob service storage)
```

## How It Works

1. The blob service stores uploaded files in `/home/data/images/` inside its container
2. This directory is mounted as a volume to `./private/uploads` on the host
3. The nginx service also mounts the same directory to serve files securely
4. Files are accessed through nginx with secure links that expire after 30 minutes

## Deployment

1. Make sure you have initialized the database:
   ```bash
   cd ../blob
   sqlx migrate run
   ```

2. Start the services:
   ```bash
   docker-compose -f blob.yml up -d
   ```

3. The blob service will be available at `http://localhost:3002`
4. Files will be served through nginx at `http://localhost:8080`

## File Access Security

Files are served through nginx with secure links that include:
- A token generated with MD5 hash
- An expiration timestamp
- The client's IP address

This ensures that file URLs cannot be shared publicly and expire after 30 minutes.

## Volume Persistence

Uploaded files are stored in the `./private/uploads` directory and will persist between container restarts.