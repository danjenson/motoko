# source: https://tinyurl.com/kj45z8l
KEY_DIR="$HOME/.keys"

# root CA
openssl genrsa -des3 -out $KEY_DIR/rootCA.key 4096
openssl req \
  -x509 \
  -new \
  -nodes \
  -key $KEY_DIR/rootCA.key \
  -sha256 \
  -days 1024 \
  -out $KEY_DIR/rootCA.pem \
  -subj "/CN=localhost/C=US/ST=CA/L=San Francisco/O=motoko/OU=ai"

# server certs
sudo openssl req \
  -new \
  -sha256 \
  -nodes \
  -out $KEY_DIR/server.csr \
  -newkey rsa:4096 \
  -keyout $KEY_DIR/server.key \
  -config server.csr.cnf
sudo openssl x509 \
  -req \
  -in $KEY_DIR/server.csr \
  -CA $KEY_DIR/rootCA.pem \
  -CAkey $KEY_DIR/rootCA.key \
  -out $KEY_DIR/server.crt \
  -CAcreateserial \
  -days 365 \
  -sha256 \
  -extfile v3.ext
