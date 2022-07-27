@REM test get balance
cargo run  -- account getbalance -u http://172.25.210.112:9999 0xfBd4505Ab8FA67747E5bfba0F495205c633F18b6 


@REM test deploy once
cargo run -- contract deploy -u http://172.25.210.112:9999 -g .\src\example\deploy_contract.json


@REM test call once
cargo run -- contract call -u http://172.25.210.112:9999 -g .\src\example\call_contract.json


@REM test query
cargo run -- contract query -u http://172.25.210.112:9999 -g .\src\example\query_contract.json


@REM test deploy 1000 times
cargo run -- contract deploy -u http://172.25.210.112:9999 -g .\src\example\deploy_contract.json -c 1000


@REM test call 1000 times
cargo run -- contract call -u http://172.25.210.112:9999 -g .\src\example\call_contract.json -c 1000

