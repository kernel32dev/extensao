openssl genpkey -algorithm RSA -out key.rsa
openssl req -x509 -key key.rsa -out cert.pem -days 365
