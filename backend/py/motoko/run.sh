if [[ $1 = 'test' ]]
then sudo go run cmd/motoko/main.go -tls_cert ~/.keys/server.crt -tls_key ~/.keys/server.key -database motoko_test -webroot ../website/dist
else sudo go run cmd/motoko/main.go -tls_cert ~/.keys/server.crt -tls_key ~/.keys/server.key -database motoko -webroot ../website/dist
fi 
