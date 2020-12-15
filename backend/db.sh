#!/usr/bin/zsh

if [ "$1" = "connect" ]; then
  psql \
    --host=motoko-free-tier.cpybpfl4z4kw.us-west-1.rds.amazonaws.com \
    --port=5432 \
    --username=motoko \
    --password \
    --dbname=postgres
elif [ "$1" = "graph" ]; then
  pip install eralchemy
  eralchemy -i 'postgres://postgres@localhost/motoko' -o models.pdf
else
  echo "usage: ./db.sh [connect|graph]"
fi
