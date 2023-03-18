FROM docker.io/library/node:current-alpine
ADD dist /app/dist
ADD package.json /app/
WORKDIR /app
RUN npm install
ENTRYPOINT [ "npm", "run", "start" ]
