
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

### exposing a port.
```Dockerfile
EXPOSE <PORT> (this is used just a reference for users to know that this container expects port 3000, it doesn't affect how to container image behaves)

```

### multistaging.
```Dockerfile
FROM package as <name> (this will create a stage that will be overritten later)
```

### Copying from a stage.
```Dockerfile
COPY --from=<name> from to
```

### Copying as independent layer
```Dockerfile
COPY --link from to (makes the copied layer completely independent from the layers before it)
```

### ARGS.
```Dockerfile
ARG NAMECAMELCASED=VALUE (lets you pass variables at build time via docker build. Think of it like function parameters for your Dockerfile.)
```

### HEREDOCS.
This is a newer Docker syntax that lets you write multi-line commands cleanly without the ugly backslash chain.
```Dockerfile
# syntax=docker/dockerfile:1 (at beggining)
RUN <<EOF
cmd 1 
cmd 2 
cmd 3 
...etc
EOF
```

## Docker Secrets.
There are two contexts where secrets work differently — BuildKit secrets (build time) and Swarm/Compose secrets (run time).

### 1. Build time.
```Dockerfile
# syntax=docker/dockerfile:1 (at beggining)
```
Then mount the secret temporarily during that one RUN step:
```Dockerfile
RUN --mount=type=secret,id=npm_token \
    NPM_TOKEN=$(cat /run/secrets/npm_token) \
    npm install
```
and pass it at build time.
```bash
docker build --secret id=npm_token,src=./.npmrc .
```

### 2. Run time.
-- docker compose (see this later)

## Most used docker commands.
```bash
docker run 
=>
    -d => for detach means it runs the container in the background.
    --env => to specify an environment variable.
    --interactive, -i, --tty, -t ==> runs a tty interactive shell inside the container.
    --mount, --volume, -v ==> to mount and load storage volumes.
    --name ==> to specify a container name.
    --network, --net => to run a container in a specific docker network.
    -p => to connect a port between the host and the container
    --rm ==> when the container stops it's removed.
```
