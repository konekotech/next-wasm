FROM node:22.3.0

WORKDIR /app/

COPY ./package.json ./
COPY ./package-lock.json ./

RUN npm install