
.PHONY: 

dependency-up:
	@docker run -d --name postgres \
    	-p 5432:5432 \
    	-e POSTGRES_USER=postgres \
    	-e POSTGRES_PASSWORD=postgres_password \
    	-e POSTGRES_DB=postgres \
    	postgres:latest

dependency-down:
	@docker stop postgres && docker rm postgres
