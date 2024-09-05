FROM docker.io/library/node:current-alpine
ADD dist /app/dist
ADD package.json /app/
ADD LICENSE /app/
WORKDIR /app
RUN yarn install
ENTRYPOINT [ "yarn", "start" ]
