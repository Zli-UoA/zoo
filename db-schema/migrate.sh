#!/bin/bash

export $(grep -v '^#' .env | xargs)
npm i -D prisma-erd-generator @mermaid-js/mermaid-cli
npx prisma migrate dev --name init
