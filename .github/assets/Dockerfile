FROM ubuntu

COPY dist/mev /usr/local/bin

# Copy licenses
COPY LICENSE-* ./

# Copy the genesis files
ADD etc/dev-genesis.json ./etc/dev-genesis.json
ADD etc/mev-genesis.json ./etc/mev-genesis.json

EXPOSE 30303 30303/udp 9001 8545 9000 8546
ENTRYPOINT ["/usr/local/bin/mev"]
