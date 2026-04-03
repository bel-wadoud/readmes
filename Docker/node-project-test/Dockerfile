FROM node:25.8-alpine3.22

WORKDIR /usr/share/app

ENV NODE_ENV production

COPY package*.json ./

RUN npm install

USER node

COPY --chown=node:node ./index.js .

EXPOSE 3000

CMD ["node", "index.js"]
