# Canal Builder

## WebAssembly Implementation of the Canal Builder Game

Create the Docker image with:
````
docker build --tag=rust-wasm .
````

Run the Docker container:
```` bash
docker run -p 8080:8080 -it --rm  -e USER='User Name'  -v "$PWD":/app --name wasm-package rust-wasm:latest /bin/bash
````


