if [[ $1 =~ 'test' ]]
then python server.py -db motoko_test
else python server.py -db motoko
fi 
