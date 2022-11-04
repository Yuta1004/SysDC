# 1. Build app
FROM node:18-alpine3.15 as builder

WORKDIR /workdir

COPY node_modules* node_modules
COPY orig_modules* orig_modules
COPY src src
COPY public public
COPY package.json tsconfig.json .

RUN npm cache verify && \
    npm install && \
    npm run build && \
    npm install -g serve

# 2. Build container 
FROM node:18-alpine3.15

COPY --from=builder /workdir/build /workdir

WORKDIR /workdir

RUN npm install -g serve

CMD serve
