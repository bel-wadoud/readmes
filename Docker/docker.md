
# Basics commands and container management.
### runs a docker image
```
docker run <image-name>
```
#### properties
```
docker run
    --interactive ==> creates a shell inside the created container
    --rm ==> removes the image directly after running it.
    --tty ==> gives the user a real shell simulator.
    --name <container-name> ==> gives a name to the container.
    --it ==> interactive + tty
```

### lists all available images.
```
docker pst
    -a ==> for all (inactive)
```

### starts a suspended container in ur device.
```
docker start <container-name>
```

### attaches to a running container's shell.
```
docker attach <container-name>
```

# docker volumes
### create a new volume
```
docker volume create <volume-name>
```

### runs a container with volumes.
```
docker run -it --mount /path/on/host:/path/on/container <container-name>
```
**Note:** docker automatically creates the /path/to/file

### Example: saving postgress database into a volume.
```
dock
docker run -d \ ==> for detach
    --name postgress-db \
    -e POSTGRESS_PASSWORD modpass \
    -v <volume-name> \
    -p 5432:5432 \
```

# Docker Files
[https://docs.docker.com/reference/dockerfile/][Dockerfile] full instructions and convintions.

### specifies the image we are going to use.
```Dockerfile
FROM <image-name>
```

### runs a terminal command inside the container.
```Dockerfile
RUN <command-name>
```

### Runs a dockerfile (builds the container)
```
docker build . (or Dockerfile path)
docker build -t name:tag (to avoid hash names.)
```

### copies my cwd into the container.
```Dockerfile
COPY . .
```

### specifies default command to run when running the container.
```Dockerfile
CMD ["npm", "run", "dev"] # Example.
```

### specifies the working directory inside the image.
```Dockerfile
WORKDIR /path/to/dir/
```

### setting an environment variable.
```Dockerfile
ENV key value
```

# docker ignore file.
```
extension: .dockerignore
```

### switching to another user.
```Dockerfile
USER <username>
```
