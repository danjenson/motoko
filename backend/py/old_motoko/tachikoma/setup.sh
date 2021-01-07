sudo dnf install mongodb mongodb-devel mongodb-server mongo-tools
sudo systemctl enable mongodb
sudo systemctl start mongodb
pip install -r requirements.txt
./test.sh
