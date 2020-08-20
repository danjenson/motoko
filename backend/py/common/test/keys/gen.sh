openssl req \
  -new \
  -newkey rsa:4096 \
  -x509 \
  -sha256 \
  -days 100000 \
  -nodes \
  -out test.crt \
  -keyout test.key \
  -subj "/C=US/ST=CA/L=San Francisco/O=motoko/OU=ai/CN=bufnet/emailAddress=daniel.a.jenson@gmail.com"
