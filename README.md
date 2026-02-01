### Install Rust and MySQL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
sudo apt install mysql-server mysql-client libmysqlclient-dev
sudo systemctl start mysql
sudo systemctl enable mysql

### Create .env fil with your configuration
cp env.example .env

### Install sqlx-cli for migrations
cargo install sqlx-cli --features mysql

### Run migrations
sqlx migrate run --database-url "mysql://portfolio_user:password@localhost/portfolio"

### Build and run
cargo run

### Production
```
cargo build --release
cp ./target/release/portfolio-backend .
```

### To run it as a service
- Create portfolio-backend.service file into /lib/systemd/system

```
sudo systemctl daemon-reload
sudo systemctl start portfolio-backend
sudo systemctl enable portfolio-backend
```