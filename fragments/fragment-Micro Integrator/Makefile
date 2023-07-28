all: build run

build:
	mvn clean install
	cp target/capp/*.car mi-home/carbonapps/ 

run:
	docker-compose up --build -d 
