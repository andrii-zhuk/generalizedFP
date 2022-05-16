# TODO: we need to buidl wasm-pack here;
# from webApp: RUN wasm-pack build ../computer/ --out-dir ../webApp/pkg --out-name index

FROM node:slim as frontend

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/webApp
RUN yarn install
RUN yarn build

FROM nginx:alpine
COPY --from=frontend /usr/src/app/webApp/dist /usr/share/nginx/html
EXPOSE 8080