FROM node:22-alpine AS build

WORKDIR /app

COPY frontend/package*.json ./
RUN npm ci

COPY frontend ./
ARG VITE_API_BASE=
ENV VITE_API_BASE=$VITE_API_BASE
RUN npm run build
RUN node -e "const fs=require('fs');const path=require('path');const htmlPath='dist/index.html';let html=fs.readFileSync(htmlPath,'utf8');html=html.replace(/<link rel=\"stylesheet\"[^>]*href=\"([^\"]+\\.css)\"[^>]*>/,(_,href)=>{const cssPath=path.join('dist',href.replace(/^\\//,''));const css=fs.readFileSync(cssPath,'utf8');return '<style>'+css.replace(/<\\/style/gi,'<\\\\/style')+'</style>';});fs.writeFileSync(htmlPath,html);"

FROM nginx:1.27-alpine

COPY server/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80
