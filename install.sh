#!/bin/sh
git pull
cargo build --release
cp target/release/pecunia /usr/local/bin/
cp cfg/pecunia.prd.json /etc/opt/pecunia/pecunia.json
cp cfg/log4rs.prd.yaml /etc/opt/pecunia/log4rs.yaml
