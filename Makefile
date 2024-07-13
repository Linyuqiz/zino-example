
.PHONY: 

dependency-up:
	@docker run -d --name postgres \
    	-p 5432:5432 \
    	-e POSTGRES_USER=postgres \
    	-e POSTGRES_PASSWORD=smcddNr2mrpwgYvO6ICRLPFfLFd27WySGN9a7a9JrsYP3tIP \
    	-e POSTGRES_DB=data_cube \
    	postgres:latest

dependency-down:
	@docker stop postgres && docker rm postgres
