FROM dshaff/scatspotter-build:latest as builder
WORKDIR /app
COPY ./client /app/client
WORKDIR /app/client
# the alternative is trying to install nodejs, which is a big pain
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.3.6/tailwindcss-linux-arm64 && mv tailwindcss-linux-arm64 tailwindcss && chmod +x tailwindcss
ENV PATH /app/client:$PATH
RUN trunk build --release 

FROM python:3.11.7-slim-bookworm

RUN apt-get update && apt-get install -y libssl-dev build-essential

WORKDIR /app
ENV POETRY_HOME=/opt/poetry

RUN python -m venv ${POETRY_HOME}
RUN ${POETRY_HOME}/bin/pip install poetry 

COPY . .
COPY --from=builder /app/client/dist /app/dist
RUN ${POETRY_HOME}/bin/poetry install

EXPOSE 8080

CMD ["/opt/poetry/bin/poetry", "run", "granian", "--interface", "asgi", "app:app", "--host", "0.0.0.0", "--port", "8080"]
