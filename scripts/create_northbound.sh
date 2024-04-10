mkdir $(pwd)/dbs
sudo ovsdb-tool create $(pwd)/dbs/nb.db $(pwd)/ovn-schema/ovn-nb.ovsschema