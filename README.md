# Tuno Socket Server

Dependencies for Ubuntu:
```sh
sudo apt install libssl-dev pkg-config protobuf-compiler
```

In case it doesn't work with letsencrypt auto-generated files:
```sh
sudo openssl pkcs8 -topk8 -nocrypt -in /etc/letsencrypt/live/tuno.media/privkey.pem -out /etc/letsencrypt/live/tuno.media/privkey-pkcs8.pem
```

## Deploy on prod as a systemd service

1. Create a specific user to run the Tuno service:
```sh
sudo useradd -r -s /bin/false tuno-distributor
```

2. Create necessary directories
```sh
sudo mkdir -p /opt/tuno-distributor
sudo mkdir -p /opt/tuno-distributor/media
```

3. Build and install the application
```sh
cargo build --release
sudo cp ./target/release/tuno-cli /opt/tuno-distributor/
```

4. Configure certificate access
```sh
# Create a directory for certificate copies
sudo mkdir -p /opt/tuno-distributor/certs

# Copy the certificates (adjust paths if needed)
sudo cp /etc/letsencrypt/live/tuno.media/fullchain.pem /opt/tuno-distributor/certs/
sudo cp /etc/letsencrypt/live/tuno.media/privkey.pem /opt/tuno-distributor/certs/

# Adjust ownership and permissions
sudo chown -R tuno-distributor:tuno-distributor /opt/tuno-distributor/certs
sudo chmod 600 /opt/tuno-distributor/certs/*.pem
```

5. Create a configuration file
```sh
sudo cp ./config.toml /opt/tuno-distributor
```

6. Set proper permissions
```sh
sudo chown -R tuno-distributor:tuno-distributor /opt/tuno-distributor
```

7. Create a systemd service file
```sh
sudo cp ./config/tuno-distributor.service /etc/systemd/system
```

8. Enable and start the service
```sh
sudo systemctl daemon-reload
sudo systemctl enable tuno-distributor.service
sudo systemctl start tuno-distributor.service
```

#### Manual Testing

Check logs with `sudo journalctl -u tuno-distributor.service`

Test grpc implementation with `grpcui`:
```sh
grpcui -plaintext "localhost:4114"
```

## Deploy

Publish package:
```sh
iota client publish tuno
```

Export created PackageID from Published Objects:
```sh
export PKG="<PackageID>"
```

TODO: report problem with SDK's default import method
```
error[E0432]: unresolved import `iota_sdk::wallet_context::WalletContext`
 --> src/client.rs:5:5
  |
5 |     wallet_context::WalletContext
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `WalletContext` in `wallet_context`
```