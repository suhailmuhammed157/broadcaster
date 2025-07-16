# Project variables
IMAGE_NAME=broadcaster
CONTAINER_NAME=broadcaster-container
PORT=9090

# Binary name must match the one built by cargo
BINARY_NAME=broadcaster

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Run the Docker container (detached)
run:
	docker run -d --name $(CONTAINER_NAME) --env-file .env -p $(PORT):$(PORT) $(IMAGE_NAME)

# Stop the running container
stop:
	docker stop $(CONTAINER_NAME)


# Clean up (remove image and stop container)
clean: stop
	docker rmi $(IMAGE_NAME)

# Clean up (remove image and stop container)
rm: 
	docker rm $(CONTAINER_NAME)


# Rebuild and run (useful during development)make
restart: stop build rm run

# Show logs from the running container
logs:
	docker logs -f $(CONTAINER_NAME)

# Shell into the running container (for debugging)
shell:
	docker exec -it $(CONTAINER_NAME) /bin/bash