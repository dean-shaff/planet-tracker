image-build:
	docker build -t dshaff/planet-tracker:latest .

image-deploy:
	docker push dshaff/planet-tracker:latest

deploy:
	git push dokku main