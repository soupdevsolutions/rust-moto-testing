start:
  docker run --rm -p 5000:5000 -d --name moto motoserver/moto:latest
stop:
  docker stop moto
