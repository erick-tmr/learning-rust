# learning-rust

## Creating a new application using cargo from docker container

This should bind mount the working directory into the `/tmp` directory inside the container.  
`docker run -it -v "$(pwd)":/tmp --rm rust:latest`

To create a new project  
`cargo new /tmp/{{project-name}}`

## Running the application

Building from the Dockerfile (naming the image as 'learning-rust')  
`docker build -t learning-rust .`

Running the app (naming the container as 'learning-rust1')  
`docker run -it --rm --name learning-rust1 learning-rust`
