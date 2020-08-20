echo ----------------------------------------
echo '    INSTALLING REQUIRED PACKAGES'
echo ----------------------------------------
echo
sudo dnf install \
  mariadb \
  mariadb-server \
  golang \
  python3 \
  protobuf \
  protobuf-devel \
  protobuf-compiler
sudo systemctl enable mariadb
sudo systemctl start mariadb
echo
echo DONE
echo
echo ----------------------------------------
echo '      SETTING UP MYSQL DATABASES'
echo ----------------------------------------
echo 'CREATE DATABASE IF NOT EXISTS motoko' | mysql -u root
echo 'CREATE DATABASE IF NOT EXISTS motoko_test' | mysql -u root
echo
echo setting timezone to be PST
sudo -- sh -c "printf \"\n[msyqld]\ndefault_time_zone='-08:00'\" >> /etc/my.cnf"
mysql -u root motoko < internal/acctmgr/init.sql
mysql -u root motoko_test < internal/acctmgr/init.sql
echo creating a test account: motoko.kusanagi@sector9.jp, Motoko Kusanagi
echo 'call new_account("motoko.kusanagi@sector9.jp", "Motoko Kusanagi", @apiKey)' \
  | mysql -u account_manager motoko_test | tail -1 > ../common/test/keys/test_api_key.txt
echo
echo DONE
echo
echo ----------------------------------------
echo '         GENERATING TLS KEYS'
echo ----------------------------------------
echo
./ssl.sh
echo
echo DONE
echo
echo ----------------------------------------
echo '           RUNNING TESTS'
echo ----------------------------------------
echo
./test.sh
echo
echo DONE
echo
echo add rootCA.pem to trusted Authorities in browser
