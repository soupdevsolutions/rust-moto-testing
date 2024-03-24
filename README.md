# Using moto from Rust to mock AWS calls

This shows an example on how to use [moto](https://pypi.org/project/moto/) in server mode and use it for testing purposes from Rust applications.  
You can read the full article [here](https://www.soup.dev/post/using-moto-from-rust-to-mock-aws-calls).  

## Usage

To start the moto server, run:
```bash
docker run --rm -p 5000:5000 -d --name moto motoserver/moto:latest
```

Then, you can test your application normally by running:
```bash
cargo test
```

Finally, stop the Docker container by running:
```bash
docker stop moto
```
